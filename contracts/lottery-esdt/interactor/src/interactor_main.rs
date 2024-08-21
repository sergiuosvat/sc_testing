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
    let error = ExpectError(4, "Invalid burn percentage!");
    match cmd.as_str() {
        "deploy" => interact.deploy().await,
        "start" => interact.start(error).await,
        "createLotteryPool" => interact.create_lottery_pool().await,
        "buy_ticket" => interact.buy_ticket().await,
        "determine_winner" => interact.determine_winner().await,
        "status" => interact.status().await,
        "set_roles" => interact.set_roles().await,
        "getLotteryInfo" => interact.lottery_info().await,
        "getLotteryWhitelist" => interact.lottery_whitelist().await,
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
        interactor.register_wallet(test_wallets::ivan());
        interactor.register_wallet(test_wallets::carol());
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/lottery-esdt.mxsc.json",
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
            .gas(50_000_000u64)
            .typed(proxy::LotteryProxy)
            .init()
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }

    async fn start(&mut self, error: ExpectError<'_>) {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"SRG-10c87e"[..]);
        let ticket_price = BigUint::<StaticApi>::from(1u128);
        let opt_total_tickets = Option::Some(2u32);
        let opt_deadline = Option::Some(2000000u64);
        let opt_max_entries_per_user = Option::Some(1u32);
        let prize_distribution_data: &[u8] = &[75,25];
        let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
        let admin: &Address = &self.wallet_address;
        let mut whitelist = ManagedVec::new();
        let managed_address = ManagedAddress::from_address(admin);
        whitelist.push(managed_address);
        let opt_whitelist = Option::Some(whitelist);
        let opt_burn_percentage = OptionalValue::Some(BigUint::<StaticApi>::from(200u128));



        self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::LotteryProxy)
            .start(lottery_name, token_identifier, ticket_price, opt_total_tickets, opt_deadline, opt_max_entries_per_user, opt_prize_distribution, opt_whitelist, opt_burn_percentage)
            .returns(error)
            .prepare_async()
            .run()
            .await;
    }

    async fn create_lottery_pool(&mut self) {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"SRG-10c87e"[..]);
        let ticket_price = BigUint::<StaticApi>::from(1u128);
        let opt_total_tickets = Option::Some(2u32);
        let opt_deadline = Option::Some(20u64);
        let opt_max_entries_per_user = Option::Some(1u32);
        let prize_distribution_data: &[u8] = &[75,25];
        let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
        let admin: &Address = &self.wallet_address;
        let mut whitelist = ManagedVec::new();
        let managed_address = ManagedAddress::from_address(admin);
        whitelist.push(managed_address);
        let opt_whitelist = Option::Some(whitelist);
        let opt_burn_percentage = OptionalValue::Some(BigUint::<StaticApi>::from(101u128));

        self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .create_lottery_pool(lottery_name, token_identifier, ticket_price, opt_total_tickets, opt_deadline, opt_max_entries_per_user, opt_prize_distribution, opt_whitelist, opt_burn_percentage)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

    }

    async fn buy_ticket(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let lottery_name = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .buy_ticket(lottery_name)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn determine_winner(&mut self) {
        let lottery_name = ManagedBuffer::new_from_bytes(&b""[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::LotteryProxy)
            .determine_winner(lottery_name)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn status(&mut self) {
        let lottery_name = ManagedBuffer::new_from_bytes(&b""[..]);

        self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .status(lottery_name)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

    }

    async fn set_roles(&mut self) {
        let token_identifier = TokenIdentifier::from_esdt_bytes(&b"SRG-10c87e"[..]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(90_000_000u64)
            .typed(proxy::LotteryProxy)
            .set_roles(token_identifier)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn lottery_info(&mut self) {
        let lottery_name = ManagedBuffer::new_from_bytes(&b""[..]);

        self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .lottery_info(lottery_name)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn lottery_whitelist(&mut self) {
        let lottery_name = ManagedBuffer::new_from_bytes(&b""[..]);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::LotteryProxy)
            .lottery_whitelist(lottery_name)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

}

// #[tokio::test]
// async fn test_deploy() {
//     let mut interact = ContractInteract::new().await;
//     //interact.deploy().await;
//     interact.set_roles().await;
//     //interact.start(ExpectError(4, "Invalid burn percentage!")).await;
// }
