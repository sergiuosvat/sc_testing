#![allow(non_snake_case)]

mod proxy;

use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";


#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "issue_fungible_token_mapper" => interact.issue_fungible_token_mapper().await,
        "issue_non_fungible" => interact.issue_non_fungible().await,
        "issue_fungible" => interact.issue_fungible().await,
        "set_roles" => interact.set_roles().await,
        "create_nft" => interact.create_nft().await,
        "update_attributes" => interact.update_attributes().await,
        "send_nft" => interact.send_nft().await,
        "nft_token_id" => interact.nft_token_id().await,
        "test_token_mapper" => interact.test_token_mapper().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/update-attributes.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }

    async fn deploy(&mut self) {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(proxy::UpdateAttributesProxy)
            .init()
            .code(&self.contract_code)
            .gas(50_000_000)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    async fn issue_fungible_token_mapper(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(50000000000000000u64);

        let token_name = ManagedBuffer::new_from_bytes(&b"Testing"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"TST"[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .issue_fungible_token_mapper(token_name, token_ticker)
            .gas(70_000_000)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn issue_non_fungible(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(50000000000000000u64);

        let token_name = ManagedBuffer::new_from_bytes(&b"Test2"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"TST2"[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .issue_non_fungible(token_name, token_ticker)
            .egld(egld_amount)
            .gas(70_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn issue_fungible(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(50000000000000000u64);

        let token_name = ManagedBuffer::new_from_bytes(&b"Testing"[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b"TST"[..]);
        let initial_supply = 5000000000000000000u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .issue_fungible(token_name, token_ticker, initial_supply)
            .egld(egld_amount)
            .gas(70_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_roles(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .set_roles()
            .gas(70_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn create_nft(&mut self) {
        let to = bech32::decode("erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"); //alice

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .create_nft(to)
            .gas(40_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn update_attributes(&mut self) {
        let token_id = "TST-ebac77";
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(5000000000000000000u64);

        let properties = FungibleTokenProperties {
            num_decimals: 18usize,
            can_freeze: false,
            can_wipe: true,
            can_pause: false,
            can_mint: true,
            can_burn: true,
            can_change_owner: true,
            can_upgrade: true,
            can_add_special_roles: true,
        };
        let mut new_attributes = ManagedBuffer::new();
        new_attributes.append_bytes(&[properties.num_decimals as u8]);
        new_attributes.append_bytes(&[properties.can_freeze as u8]);
        new_attributes.append_bytes(&[properties.can_wipe as u8]);
        new_attributes.append_bytes(&[properties.can_pause as u8]);
        new_attributes.append_bytes(&[properties.can_mint as u8]);
        new_attributes.append_bytes(&[properties.can_burn as u8]);
        new_attributes.append_bytes(&[properties.can_change_owner as u8]);
        new_attributes.append_bytes(&[properties.can_upgrade as u8]);
        new_attributes.append_bytes(&[properties.can_add_special_roles as u8]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .update_attributes(new_attributes)
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .gas(50_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn send_nft(&mut self) {
        let to = bech32::decode("erd1k2s324ww2g0yj38qn2ch2jwctdy8mnfxep94q9arncc6xecg3xaq6mjse8"); //carol
        let nonce = 1u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .send_nft(to, nonce)
            .gas(50_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn nft_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .nft_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn test_token_mapper(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::UpdateAttributesProxy)
            .test_token_mapper()
            .returns(ReturnsResult)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}

#[tokio::test]
async fn test_deploy() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
}

#[tokio::test]
async fn test_issue_fungible() {
    let mut interact = ContractInteract::new().await;
    interact.issue_fungible().await;
}

#[tokio::test]
async fn test_issue_fungible_token_mapper() {
    let mut interact = ContractInteract::new().await;
    interact.issue_fungible_token_mapper().await;
}

#[tokio::test]
async fn test_issue_non_fungible() {
    let mut interact = ContractInteract::new().await;
    interact.issue_non_fungible().await;
    interact.test_token_mapper().await;
}

#[tokio::test]
async fn test_get_token_mapper() {
    let mut interact = ContractInteract::new().await;
    interact.test_token_mapper().await;
}

#[tokio::test]
async fn test_create_nft() {
    let mut interact = ContractInteract::new().await;
    interact.create_nft().await;
    interact.nft_token_id().await;
}

#[tokio::test]
async fn test_send_nft() {
    let mut interact = ContractInteract::new().await;
    interact.send_nft().await;
}

#[tokio::test]
async fn test_update_attributes() {
    let mut interact = ContractInteract::new().await;
    interact.set_roles().await;
    interact.update_attributes().await;
}
