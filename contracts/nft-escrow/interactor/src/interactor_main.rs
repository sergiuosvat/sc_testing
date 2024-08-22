#![allow(non_snake_case)]
#![allow(dead_code)]

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
const IVAN_ADDRESS: &str = "erd13x29rvmp4qlgn4emgztd8jgvyzdj0p6vn37tqxas3v9mfhq4dy7shalqrx";
const WANTED_ADDRESS_STRING: &str = "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx";
const TOKEN_ID: &str = "INTERNS-c9325f";


#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let _cmd = args.next().expect("at least one argument required");
    let _interact = ContractInteract::new().await;
    // match cmd.as_str() {
    //     "deploy" => interact.deploy().await,
    //     "escrow" => interact.escrow().await,
    //     "cancel" => interact.cancel().await,
    //     "accept" => interact.accept().await,
    //     "getCreatedOffers" => interact.get_created_offers().await,
    //     "getWantedOffers" => interact.get_wanted_offers().await,
    //     "created_offers" => interact.created_offers().await,
    //     "wanted_offers" => interact.wanted_offers().await,
    //     "offers" => interact.offers().await,
    //     _ => panic!("unknown command: {}", &cmd),
    // }
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
        let wallet_address = interactor.register_wallet(test_wallets::ivan());
        
        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/nft-escrow.mxsc.json",
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
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
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

    async fn escrow_succes(&mut self, token_id: String, token_nonce: u64, token_amount: BigUint<StaticApi>, 
                    wanted_nft: TokenIdentifier<StaticApi>, wanted_nonce: u64, wanted_address: &Bech32Address) -> u32 { 
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .escrow(wanted_nft, wanted_nonce, wanted_address)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result Escrow Success: {:?}", response);
        response
    }

    async fn escrow_fail(&mut self, token_id: String, token_nonce: u64, token_amount: BigUint<StaticApi>, 
                    wanted_nft: TokenIdentifier<StaticApi>, wanted_nonce: u64, wanted_address: &Bech32Address, expected_result: ExpectError<'_>) { 
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .escrow(wanted_nft, wanted_nonce, wanted_address)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn cancel(&mut self, offer_id: u32) {

        self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .cancel(offer_id)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

    }

    async fn cancel_failed(&mut self, offer_id: u32, expected_result: ExpectError<'_>) {

        self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .cancel(offer_id)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

    }

    async fn cancel_failed_adress(&mut self, offer_id: u32, expected_result: ExpectError<'_>) {

        let wallet_address = self.interactor.register_wallet(test_wallets::carol());
        self
            .interactor
            .tx()
            .from(wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .cancel(offer_id)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

    }

    async fn accept_success(&mut self, token_id: String, token_nonce: u64, token_amount:  BigUint<StaticApi>, offer_id: u32) {
        let user = self.interactor.register_wallet(test_wallets::bob());
        let response = self
            .interactor
            .tx()
            .from(user)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .accept(offer_id)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, BigUint::from(token_amount)))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn accept_fail(&mut self, token_id:  String, token_nonce: u64, token_amount:  BigUint<StaticApi>, offer_id: u32, expected_result: ExpectError<'_>) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .accept(offer_id)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, BigUint::from(token_amount)))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn accept_fail_address(&mut self, token_id: String, token_nonce: u64, token_amount:u128, offer_id: u32, expected_result: ExpectError<'_>) {
        //let wallet_address = self.interactor.register_wallet(test_wallets::bob());
        let user = self.interactor.register_wallet(test_wallets::alice());
        let response = self
            .interactor
            .tx()
            .from(user)
            .to(self.state.current_address())
            .gas(NumExpr("30,000,000"))
            .typed(proxy::NftEscrowContractProxy)
            .accept(offer_id)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, BigUint::from(token_amount)))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn get_created_offers(&mut self, address: Bech32Address) {
        // let address = bech32::decode("erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::NftEscrowContractProxy)
            .get_created_offers(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result created offers: {result_value:?}");
    }

    async fn get_wanted_offers(&mut self, address: Bech32Address) {
        // let address = bech32::decode("erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::NftEscrowContractProxy)
            .get_wanted_offers(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result wanted offers: {result_value:?}");
    }

    async fn created_offers(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::NftEscrowContractProxy)
            .created_offers(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn wanted_offers(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::NftEscrowContractProxy)
            .wanted_offers(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn offers(&mut self, id: u32) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::NftEscrowContractProxy)
            .offers(id)
            .returns(ReturnsResultUnmanaged)
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
async fn test_escrow_nonce_zero() {
    let mut interact = ContractInteract::new().await;
    let token_id = String::from("NICU-970292");
    let token_nonce = 0u64;
    let token_amount = BigUint::<StaticApi>::from(5u128);
    let wanted_nft = TokenIdentifier::from_esdt_bytes(&b""[..]);
    let wanted_nonce = 10u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    interact.escrow_fail(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address, ExpectError(4, "ESDT is not an NFT")).await;
}

#[tokio::test]
async fn test_escrow_value_diff_one() {
    let mut interact = ContractInteract::new().await;
    let token_id = String::from(TOKEN_ID); 
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(2u128);
    let wanted_nft = TokenIdentifier::from_esdt_bytes(&b""[..]);
    let wanted_nonce = 10u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    interact.escrow_fail(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address, ExpectError(4, "ESDT is not an NFT")).await;
}

#[tokio::test]
async fn test_escrow_wanted_nonce_zero() {
    let mut interact = ContractInteract::new().await;
    let token_id = String::from(TOKEN_ID); // "BSK-476470" 
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::from_esdt_bytes(&b""[..]);
    let wanted_nonce = 0u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    interact.escrow_fail(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address, ExpectError(4, "Wanted ESDT is not an NFT")).await;
}

#[tokio::test]
async fn test_escrow_diff_addresses() {
    let mut interact = ContractInteract::new().await;
    let token_id = String::from(TOKEN_ID); // "BSK-476470" 
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::from_esdt_bytes(&b""[..]);
    let wanted_nonce = 10u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(test_wallets::ivan().address().to_string()));
    interact.escrow_fail(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address, ExpectError(4, "Wanted address should not be the same as the caller")).await;
}

#[tokio::test]
async fn test_escrow_success() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id = String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&b"MICE-9e007a"[..]);
    let wanted_nonce = 106u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    interact.escrow_succes(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address).await;

    let ivan_address = Bech32Address::from_bech32_string(String::from(IVAN_ADDRESS));
    let wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));

    interact.get_created_offers(ivan_address).await;
    interact.get_wanted_offers(wanted_address).await;
}

#[tokio::test]
async fn test_escrow_success_twice() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id = String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&b"MICE-9e007a"[..]);
    let wanted_nonce = 106u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    interact.escrow_succes(token_id.clone(), token_nonce, token_amount.clone(), wanted_nft.clone(), wanted_nonce, wanted_address).await;
    interact.escrow_succes(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address).await;

    let ivan_address = Bech32Address::from_bech32_string(String::from(IVAN_ADDRESS));
    let wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));

    interact.get_created_offers(ivan_address).await;
    interact.get_wanted_offers(wanted_address).await;
}

#[tokio::test]
async fn test_accept_fail_offer_does_not_exist(){
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id = String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount =  BigUint::<StaticApi>::from(2u128);
    let offer_id = 9999;
    interact.accept_fail(token_id, token_nonce, token_amount, offer_id, ExpectError(4, "Offer does not exist")).await;
}

#[tokio::test]
async fn test_unauthorized_acceptance() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id = String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&b"MICE-9e007a"[..]);
    let wanted_nonce = 106u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from("erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"));
    let offer_id = interact.escrow_succes(
        token_id.clone(), token_nonce, token_amount.clone(), wanted_nft, wanted_nonce, wanted_address
    ).await;

    println!("Offer id: {offer_id}");

    let expected_error = ExpectError(4, "Can not accept this offer");
    interact
        .accept_fail(
           token_id, token_nonce, token_amount, offer_id, expected_error
        )
        .await;
}

#[tokio::test]
async fn test_nft_does_not_match() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id =  String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount =BigUint::<StaticApi>::from(1u128);
    let wanted_nft =   TokenIdentifier::<StaticApi>::from_esdt_bytes(&"INTERNS-c9325f"[..]);
    let wanted_nonce = 1u64;
    let wanted_address = Bech32Address::from_bech32_string(String::from("erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"));
    let offer_id = interact.escrow_succes(
        token_id.clone(), token_nonce, token_amount, wanted_nft.clone(), wanted_nonce, &wanted_address
    ).await;


    println!("Offer id: {}", offer_id); 
    let token_amount = 2u128;
    let expected_error = ExpectError(4, "NFT does not match");
    interact
        .accept_fail_address(
            token_id.clone(), token_nonce, token_amount, offer_id, expected_error
        ).await;

    let toke_nonce_diff = 2u64;
    interact
        .accept_fail_address(
            token_id, toke_nonce_diff, token_amount, offer_id, ExpectError(4, "NFT does not match")
        ).await;
    
    let token_id_diff = "INTERNS-123456";
    interact
        .accept_fail_address(
            token_id_diff.to_string(), token_nonce, token_amount, offer_id, ExpectError(4, "NFT does not match")
        ).await;
    
}

#[tokio::test]
async fn test_accept_success() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id =  String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount =BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&"MICE-9e007a"[..]);
    let wanted_nonce = 2u64;
    let wanted_address = Bech32Address::from_bech32_string(String::from("erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"));
    let offer_id = interact.escrow_succes(
        token_id.clone(), token_nonce, token_amount.clone(), wanted_nft.clone(), wanted_nonce, &wanted_address
    ).await;
    println!("Offer id: {}", offer_id); 

    interact
        .accept_success(
            token_id, token_nonce, token_amount, offer_id
        ).await;
}


#[tokio::test]
async fn test_cancel_offer_not_exists() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let offer_id = 123u32;
    interact.cancel_failed(offer_id, ExpectError(4, "Offer does not exist")).await;
}

#[tokio::test]
async fn test_cancel_offer_not_owner() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id = String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&b"MICE-9e007a"[..]);
    let wanted_nonce = 106u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    let offer_id = interact.escrow_succes(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address).await;
    interact.cancel_failed_adress(offer_id, ExpectError(4, "Only the offer creator can cancel it")).await;
}

#[tokio::test]
async fn test_cancel_offer_success() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;

    let token_id = String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount = BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&b"MICE-9e007a"[..]);
    let wanted_nonce = 106u64;
    let ref wanted_address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    let offer_id = interact.escrow_succes(token_id, token_nonce, token_amount, wanted_nft, wanted_nonce, wanted_address).await;
    
    interact.cancel(offer_id).await;

    let ivan_address = Bech32Address::from_bech32_string(String::from(IVAN_ADDRESS));
    interact.get_created_offers(ivan_address).await;

}

#[tokio::test]
async fn test_created_offers_not_exists() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let address = Bech32Address::from_bech32_string(String::from(IVAN_ADDRESS));
    interact.get_created_offers(address).await;
}

#[tokio::test]
async fn test_wanted_offers_not_exists() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let address = Bech32Address::from_bech32_string(String::from(WANTED_ADDRESS_STRING));
    interact.get_wanted_offers(address).await;
}

#[tokio::test]
async fn test_multiple_accept_offer() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id =  String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount =BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&"MICE-9e007a"[..]);
    let wanted_nonce = 2u64;
    let wanted_address = Bech32Address::from_bech32_string(String::from("erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"));
    let offer_id = interact.escrow_succes(
        token_id.clone(), token_nonce, token_amount.clone(), wanted_nft.clone(), wanted_nonce, &wanted_address
    ).await;
    println!("Offer id: {}", offer_id); 

    interact
        .accept_success(
            token_id.clone(), token_nonce, token_amount.clone(), offer_id
        ).await;

    interact.accept_fail(token_id, token_nonce, token_amount, offer_id, ExpectError(4, "Can not accept this offer")).await;
}

#[tokio::test]
async fn test_multiple_offers_interaction() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let token_id =  String::from(TOKEN_ID);
    let token_nonce = 1u64;
    let token_amount =BigUint::<StaticApi>::from(1u128);
    let wanted_nft = TokenIdentifier::<StaticApi>::from_esdt_bytes(&"MICE-9e007a"[..]);
    let wanted_nonce = 2u64;
    let wanted_address = Bech32Address::from_bech32_string(String::from("erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"));
    let offer_id = interact.escrow_succes(
        token_id.clone(), token_nonce, token_amount.clone(), wanted_nft.clone(), wanted_nonce, &wanted_address
    ).await;
    let offer_id_2 = interact.escrow_succes(
        token_id.clone(), token_nonce, token_amount.clone(), wanted_nft.clone(), wanted_nonce, &wanted_address
    ).await;

    interact
        .accept_success(
            token_id, token_nonce, token_amount, offer_id
        ).await;
    
    interact.cancel(offer_id_2).await;  
}