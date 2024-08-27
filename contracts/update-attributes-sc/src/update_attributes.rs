#![no_std]
#[allow(unused_imports)]
use custom_callbacks::{CallbackProxy, CustomCallbacks};
use multiversx_sc::imports::*;
use multiversx_sc_modules::default_issue_callbacks;

mod custom_callbacks;
mod storage;

const ISSUE_COST: u64 = 50000000000000000;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait UpdateAttributes:
    default_issue_callbacks::DefaultIssueCallbacksModule
    + storage::StorageModule
    + custom_callbacks::CustomCallbacks
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn issue_fungible_token_mapper(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let payment_amount = self.call_value().egld_value().clone_value();
        self.test_token_mapper().issue_and_set_all_roles(
            payment_amount,
            token_name,
            token_ticker,
            18usize,
            None,
        )
    }

    #[payable("EGLD")]
    #[endpoint]
    fn issue_non_fungible(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        require!(self.nft_token_id().is_empty(), "Token already issued");

        let payment_amount = self.call_value().egld_value().clone_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_and_set_all_roles(
                payment_amount,
                token_name,
                token_ticker,
                EsdtTokenType::NonFungible,
                0,
            )
            .with_callback(<Self as CustomCallbacks>::callbacks(self).issue_callback())
            .async_call_and_exit()
    }

    #[payable("EGLD")]
    #[endpoint]
    fn issue_fungible(
        &self,
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: u64,
    ) {
        let caller = self.blockchain().get_caller();
        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                BigUint::from(ISSUE_COST),
                &token_name,
                &token_ticker,
                &BigUint::from(initial_supply),
                FungibleTokenProperties {
                    num_decimals: 18usize,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .callback(<Self as CustomCallbacks>::callbacks(self).token_issue_callback(&caller))
            .async_call_and_exit();
    }

    #[endpoint]
    fn set_roles(&self) {
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.nft_token_id().get(),
                [
                    EsdtLocalRole::NftUpdateAttributes,
                    EsdtLocalRole::Mint,
                    EsdtLocalRole::Burn,
                ]
                .into_iter(),
            )
            .async_call_and_exit();

        // self.tx()
        //     .to(ESDTSystemSCAddress)
        //     .typed(ESDTSystemSCProxy)
        //     .set_special_roles(
        //         &self.blockchain().get_sc_address(),
        //         &self.nft_token_id().get(),
        //         [EsdtLocalRole::NftUpdateAttributes].into_iter(),
        //     )
        //     .async_call_and_exit();
    }

    #[endpoint]
    fn create_nft(&self, to: ManagedAddress) {
        let nonce = self.send().esdt_nft_create(
            &self.nft_token_id().get(),
            &BigUint::from(1u8),
            &ManagedBuffer::new(),
            &BigUint::from(0u8),
            &ManagedBuffer::new(),
            &ManagedBuffer::from(b"common"),
            &ManagedVec::new(),
        );

        self.tx()
            .to(to)
            .single_esdt(&self.nft_token_id().get(), nonce, &BigUint::from(1u64))
            .transfer_execute();
    }

    #[payable("*")]
    #[endpoint]
    fn update_attributes(&self, new_attributes: ManagedBuffer) {
        let token = self.call_value().single_esdt();

        self.tx()
            .to(ToSelf)
            .typed(system_proxy::UserBuiltinProxy)
            .nft_update_attributes(&token.token_identifier, token.token_nonce, &new_attributes)
            .sync_call();

        self.tx()
            .to(ToCaller)
            .single_esdt(
                &token.token_identifier,
                token.token_nonce,
                &BigUint::from(1u64),
            )
            .transfer();
    }

    #[endpoint]
    fn send_nft(&self, to: ManagedAddress, nonce: u64) {
        self.tx()
            .to(to)
            .single_esdt(&self.nft_token_id().get(), nonce, &BigUint::from(1u64))
            .transfer();
    }
}
