#![allow(non_snake_case)]
#![allow(dead_code)]

mod potlock_interactor_config;
mod proxy;

use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";
const TOKEN_ID: &str = "BSK-476470";
const TOKEN_ID_2: &str = "TEST2-beba65";
const FEE_AMOUNT: u64 = 50000000000000000; // 0.05
const FEE_AMOUNT_2: u64 = 300000000000000000; // 0.03

use potlock_interactor_config::Config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    // let cmd = args.next().expect("at least one argument required");
    // let mut interact = ContractInteract::new().await;
    // match cmd.as_str() {
    //     "deploy" => interact.deploy().await,
    //     "acceptPot" => interact.accept_pot().await,
    //     "removePot" => interact.remove_pot().await,
    //     "acceptApplication" => interact.accept_application().await,
    //     "rejectDonation" => interact.reject_donation().await,
    //     "distributePotToProjects" => interact.distribute_pot_to_projects().await,
    //     "addPot" => interact.add_pot().await,
    //     "applyForPot" => interact.apply_for_pot().await,
    //     "donateToPot" => interact.donate_to_pot().await,
    //     "donateToProject" => interact.donate_to_project().await,
    //     "changeFeeForPots" => interact.change_fee_for_pots().await,
    //     "getFeeTokenIdentifier" => interact.fee_token_identifier().await,
    //     "getFeeAmount" => interact.fee_amount().await,
    //     "feePotPayments" => interact.fee_pot_proposer().await,
    //     "feeAmountAcceptPots" => interact.fee_amount_accepted_pots().await,
    //     "potDonations" => interact.pot_donations().await,
    //     "projectDonations" => interact.project_donations().await,
    //     "isAdmin" => interact.is_admin().await,
    //     "addAdmin" => interact.add_admin().await,
    //     "removeAdmin" => interact.remove_admin().await,
    //     "getAdmins" => interact.admins().await,
    //     _ => panic!("unknown command: {}", &cmd),
    // }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>,
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
    state: State,
    config: Config,
}

impl ContractInteract {
    async fn new() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(config.gateway()).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());
        interactor.register_wallet(test_wallets::carol());

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/potlock.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            contract_code,
            state: State::load_state(),
            config: Config::load_config(),
        }
    }

    async fn deploy(&mut self) {
        let admin = &self.config.admin;
        let admins = MultiValueVec::from(vec![admin.clone()]);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(proxy::PotlockProxy)
            .init(admins)
            .code(&self.contract_code)
            .gas(50_000_000)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    async fn accept_pot(&mut self, potlock_wanted: u64) {
        let admin = &self.config.admin;
        let mut potlock_id = 1u32;

        if potlock_wanted == 2 {
            potlock_id = 2u32;
        }

        let response = self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .accept_pot(potlock_id)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn accept_pot_param(&mut self, potlock_wanted: u32, expected_result: ExpectError<'_>) {
        let admin = &self.config.admin;

        self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .accept_pot(potlock_wanted)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn remove_pot(&mut self) {
        let admin = &self.config.admin;
        let potlock_id = 2u32;

        let response = self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .remove_pot(potlock_id)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_pot_param(&mut self, potlock_wanted: u32, expected_result: ExpectError<'_>) {
        let admin = &self.config.admin;

        self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .remove_pot(potlock_wanted)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn accept_application(&mut self, project_wanted: u32) {
        let admin: &Bech32Address = &self.config.admin;
        let mut project_id = 1u32;

        if project_wanted == 2 {
            project_id = 2u32;
        }

        let response = self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .accept_application(project_id)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn accept_application_params(&mut self, project_wanted: u32, expected_result: ExpectError<'_>) {
        let admin: &Bech32Address = &self.config.admin;

        self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .accept_application(project_wanted)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn reject_donation(&mut self) {
        let admin: &Bech32Address = &self.config.admin;
        let user = &self.config.pot_donor;
        let potlock_id = 0u32;

        let response = self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .reject_donation(potlock_id, user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn reject_donation_param(&mut self, potlock_wanted: u32, expected_result: ExpectError<'_>, wanted_user: Bech32Address) {
        let admin: &Bech32Address = &self.config.admin;

        self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .reject_donation(potlock_wanted, wanted_user)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn distribute_pot_to_projects(&mut self) {
        let admin: &Bech32Address = &self.config.admin;
        let potlock_id = 1u32;
        let project_percentage = MultiValueVec::from(vec![MultiValue2::from((1u32, 6_000u64)),MultiValue2::from((2u32, 4_000u64))]);

        let response = self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .distribute_pot_to_projects(potlock_id, project_percentage)
            .gas(50_000_000)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn add_pot(&mut self) {
        let pot_proposer: &Bech32Address = &self.config.pot_proposer;
        let token_id = TokenIdentifier::from_esdt_bytes(TOKEN_ID);
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(FEE_AMOUNT);

        let description = ManagedBuffer::new_from_bytes(b"Pot used for testing");
        let name = ManagedBuffer::new_from_bytes(b"My Pot");

        let response = self
            .interactor
            .tx()
            .from(pot_proposer)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .add_pot(name, description)
            .payment((token_id, token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn add_pot_params(&mut self, token_id: &str, fee: u64, expected_result: ExpectError<'_>) {
        let pot_proposer: &Bech32Address = &self.config.pot_proposer;
        let token_id = TokenIdentifier::from_esdt_bytes(token_id);
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(fee);

        let description = ManagedBuffer::new_from_bytes(b"Pot used for testing");
        let name = ManagedBuffer::new_from_bytes(b"My Pot");

        let response = self
            .interactor
            .tx()
            .from(pot_proposer)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .add_pot(name, description)
            .payment((token_id, token_nonce, token_amount))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn apply_for_pot(&mut self, potlock_wanted: u64, wanted_proposer: u64) {
        let mut project_proposer: &Bech32Address = &self.config.project_proposer;
        let mut potlock_id = 1u32;
        if potlock_wanted == 2 {
            potlock_id = 2u32;
        }

        if wanted_proposer == 2 {
            project_proposer = &self.config.project_proposer_2;
        }
        let project_name = ManagedBuffer::new_from_bytes(b"New Testing Project");
        let description = ManagedBuffer::new_from_bytes(b"Project used for testing");

        let response = self
            .interactor
            .tx()
            .from(project_proposer)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .apply_for_pot(potlock_id, project_name, description)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn donate_to_pot(&mut self) {
        let pot_donor: &Bech32Address = &self.config.pot_donor;
        let token_id = TokenIdentifier::from_esdt_bytes(TOKEN_ID);
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(3 * FEE_AMOUNT);

        let potlock_id = 1u32;

        let response = self
            .interactor
            .tx()
            .from(pot_donor)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .donate_to_pot(potlock_id)
            .payment((token_id, token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn donate_to_pot_diff_tokens(&mut self) {
        let pot_donor: &Bech32Address = &self.config.pot_donor;
        let token_id = TokenIdentifier::from_esdt_bytes(TOKEN_ID_2);
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(3 * FEE_AMOUNT);

        let potlock_id = 1u32;

        let response = self
            .interactor
            .tx()
            .from(pot_donor)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .donate_to_pot(potlock_id)
            .payment((token_id, token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn donate_to_project(&mut self) {
        let project_donor: &Bech32Address = &self.config.project_donor;
        let token_id = TokenIdentifier::from_esdt_bytes(TOKEN_ID);
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(3 * FEE_AMOUNT);

        let project_id = 1u32;

        let response = self
            .interactor
            .tx()
            .from(project_donor)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .donate_to_project(project_id)
            .payment((token_id, token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn donate_to_project_params(&mut self, token_id: &str, project_wanted: u32, expected_result: ExpectError<'_>) {
        let project_donor: &Bech32Address = &self.config.project_donor;
        let token_id = TokenIdentifier::from_esdt_bytes(token_id);
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(3 * FEE_AMOUNT);

        let response = self
            .interactor
            .tx()
            .from(project_donor)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .donate_to_project(project_wanted)
            .payment((token_id, token_nonce, token_amount))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn change_fee_for_pots(&mut self, fee_type: u64) {
        let admin: &Bech32Address = &self.config.admin;
        let token_identifier = TokenIdentifier::from_esdt_bytes(TOKEN_ID);

        let fee;

        if fee_type == 1 {
            fee = BigUint::<StaticApi>::from(FEE_AMOUNT);
        } else {
            fee = BigUint::<StaticApi>::from(FEE_AMOUNT_2);
        }

        let response = self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .change_fee_for_pots(token_identifier, fee)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn change_fee_for_pots_params(&mut self, token_identifier: &str, fee_input: u64, expected_result: ExpectError<'_>) {
        let admin: &Bech32Address = &self.config.admin;
        let token_identifier = TokenIdentifier::from_esdt_bytes(token_identifier);
        let fee = BigUint::<StaticApi>::from(fee_input);

        self
            .interactor
            .tx()
            .from(admin)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .change_fee_for_pots(token_identifier, fee)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn fee_token_identifier(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .fee_token_identifier()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn fee_amount(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .fee_amount()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn pot_donations(&mut self) {
        let potlock_id = 1u32;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .pot_donations(potlock_id)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn project_donations(&mut self, project_wanted: u64) {
        let mut project_id = 1u32;

        if project_wanted == 2 {
            project_id = 2u32;
        }

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .project_donations(project_id)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn is_admin(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .is_admin(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn add_admin(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .add_admin(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_admin(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .remove_admin(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn admins(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .admins()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_potlocks(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::PotlockProxy)
            .potlocks()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        for pot in result_value.iter() {
            println!("Result: {}", pot.name);
        }
    }
}

#[tokio::test]
async fn test_deploy() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    interact.change_fee_for_pots(1).await;
}

#[tokio::test]
async fn test_add_pot() {
    let mut interact = ContractInteract::new().await;
    interact.add_pot().await;
}

async fn test_accept_pot() {
    let mut interact = ContractInteract::new().await;
    interact.accept_pot(1).await;
}

#[tokio::test]
async fn test_view_potlocks() {
    let mut interact = ContractInteract::new().await;
    interact.add_pot().await;
    interact.get_potlocks().await;
}

#[tokio::test]
async fn test_change_fee_wrong_params() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    interact.fee_amount().await;
    interact.change_fee_for_pots(1).await;
    interact.fee_amount().await;

    // interact.change_fee_for_pots_params(
    //     "ALICE",
    //     FEE_AMOUNT,
    //     ExpectError(4, "Invalid token provided")
    // ).await;
}

#[tokio::test]
async fn test_accept_pot_not_created() {
    let mut interact = ContractInteract::new().await;
    let potlock_wanted = 0;
    interact.accept_pot_param(potlock_wanted, ExpectError(4, "Potlock doesn't exist!")).await;
}

#[tokio::test]
async fn test_remove_pot_not_created() {
    let mut interact = ContractInteract::new().await;
    let potlock_wanted = 0;
    interact.remove_pot_param(potlock_wanted, ExpectError(4, "Potlock doesn't exist!")).await;
}

#[tokio::test]
async fn test_accept_application_not_created() {
    let mut interact = ContractInteract::new().await;
    let project_wanted = 0;
    interact.accept_application_params(project_wanted, ExpectError(4, "Project doesn't exist!")).await;
}

#[tokio::test]
async fn test_reject_donation_params() {
    let mut interact = ContractInteract::new().await;
    let potlock_wanted = 0;
    let wanted_user = Bech32Address::from_bech32_string(("erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th").to_string());
    let wanted_user_2 = Bech32Address::from_bech32_string(("").to_string());
    interact.reject_donation_param(potlock_wanted, ExpectError(4, "Potlock doesn't exist!"), wanted_user).await;
    interact.reject_donation_param(1, ExpectError(4, "No donation for this user"), wanted_user_2).await;
}

#[tokio::test]
async fn test_add_pot_params(){
    let mut interact = ContractInteract::new().await;

    let token_id = "ALICE";
    interact.add_pot_params(
        token_id,
        FEE_AMOUNT,
        ExpectError(4, "Wrong token identifier for creating a pot!")
    ).await;

    interact.add_pot_params(
        TOKEN_ID,
        FEE_AMOUNT_2,
        ExpectError(4, "Wrong fee amount for creating a pot")
    ).await;
}

#[tokio::test]
async fn test_donate_to_project() {
    let mut interact = ContractInteract::new().await;

    let wanted_proposer = 1;
    let potlock_wanted = 1;
    let project_wanted : u32 = 1;
    let project_wanted_2 :u32= 2;
    let project_wanted_3 = 3;

    interact.add_pot().await;

    interact.apply_for_pot(potlock_wanted, wanted_proposer).await;
    interact.apply_for_pot(potlock_wanted, wanted_proposer).await;
    interact.accept_application(project_wanted).await;
    interact.accept_application(project_wanted_2).await;

    interact.donate_to_project().await;
    let token_id = "ALICE";
    interact.donate_to_project_params(token_id, project_wanted, ExpectError(4, "Already made a payment with a different TokenID")).await;
    interact.donate_to_project_params(token_id, project_wanted_2, ExpectError(4, "Project is not active!")).await;
    interact.donate_to_project_params(token_id, project_wanted_3, ExpectError(4, "Project doesn't exist!")).await;
    
}

#[tokio::test]
async fn test_multiple_donations() {
    let mut interact = ContractInteract::new().await;

    interact.donate_to_pot().await;
    interact.pot_donations().await;

    interact.donate_to_pot_diff_tokens().await;
    interact.pot_donations().await;

}

async fn test_multiple_donations_same_token(){
    let mut interact = ContractInteract::new().await;

    interact.donate_to_pot().await;
    interact.pot_donations().await;
    interact.donate_to_pot().await;
    interact.pot_donations().await;
}
#[tokio::test]
async fn test_multiple_accept_project(){
    let mut interact = ContractInteract::new().await;
    let potlock_wanted = 1;
    let project_wanted = 1;
    let wanted_proposer = 1;
    interact.apply_for_pot(potlock_wanted,wanted_proposer).await;
    interact.accept_application(project_wanted).await;
    interact.accept_application(project_wanted).await;
    interact.accept_application(project_wanted).await;
}

#[tokio::test]
async fn test_different_pot_project_donations(){
    let mut interact = ContractInteract::new().await;
    let potlock_wanted = 1;
    let potlock_wanted_2 = 2;
    let project_wanted = 1;
    let project_wanted_2 = 2;
    let wanted_proposer = 1;
    let wanted_proposer_2 = 2;

    interact.deploy().await;
    interact.change_fee_for_pots(1).await;
    interact.add_pot().await;
    interact.accept_pot(potlock_wanted).await;
    interact.add_pot().await;
    interact.accept_pot(potlock_wanted_2).await;

    interact.donate_to_pot().await; // set to potid 1

    interact.apply_for_pot(potlock_wanted,wanted_proposer).await;
    interact.accept_application(project_wanted).await;
    interact.apply_for_pot(potlock_wanted_2,wanted_proposer_2).await;
    interact.accept_application(project_wanted_2).await;
    interact.distribute_pot_to_projects().await;

    interact.pot_donations().await;
}

#[tokio::test]
async fn test_donate_to_project_inactive_then_activate (){
    let mut interact = ContractInteract::new().await;
    let potlock_wanted = 1;
    let project_wanted = 1;
    let wanted_proposer = 1;

    interact.deploy().await;
    interact.change_fee_for_pots(1).await;
    interact.add_pot().await;
    interact.accept_pot(potlock_wanted).await;

    interact.apply_for_pot(potlock_wanted,wanted_proposer).await;
    interact.donate_to_project_params(TOKEN_ID,project_wanted,ExpectError(4, "Project is not active!")).await;
    interact.accept_application(project_wanted).await;
    interact.donate_to_project().await;
}

#[tokio::test]
async fn test_change_fee_twice(){
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    interact.change_fee_for_pots(1).await;
    interact.fee_amount().await;
    interact.change_fee_for_pots(2).await;
    interact.fee_amount().await;
}


