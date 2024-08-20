use multiversx_sc_scenario::*;
use multiversx_sc_scenario::imports::{MxscPath, TestAddress, TestTokenIdentifier};

const _: MxscPath = MxscPath::new("../output/potlock.mxsc.json");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ADMIN_ADDRESS: TestAddress = TestAddress::new("admin");
const POT_PROPOSER_ADDRESS: TestAddress = TestAddress::new("pot_proposer");
const PROJECT_PROPOSER_ADDRESS: TestAddress = TestAddress::new("project_proposer");
const POT_DONOR_ADDRESS: TestAddress = TestAddress::new("pot_donor");
const PROJECT_DONOR_ADDRESS: TestAddress = TestAddress::new("project_donor");
const TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("POT-123456");
const DIFFERENT_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("DIFFPOT-123456");
const INITIAL_BALANCE: u64 = 2_000;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/potlock.mxsc.json", potlock::ContractBuilder);
    blockchain
            .account(OWNER_ADDRESS)
            .nonce(1)
            .account(ADMIN_ADDRESS)
            .nonce(1)
            .account(POT_PROPOSER_ADDRESS)
            .nonce(1)
            .esdt_balance(TOKEN_ID, INITIAL_BALANCE)
            .account(PROJECT_PROPOSER_ADDRESS)
            .nonce(1)
            .account(POT_DONOR_ADDRESS)
            .nonce(1)
            .esdt_balance(TOKEN_ID, INITIAL_BALANCE)
            .account(PROJECT_DONOR_ADDRESS)
            .nonce(1)
            .esdt_balance(TOKEN_ID, INITIAL_BALANCE)
            .esdt_balance(DIFFERENT_TOKEN_ID, INITIAL_BALANCE);
    blockchain
}

#[test]
fn potlock_accept_application_rs() {
    world().run("scenarios/potlock-accept-application.scen.json");
}

#[test]
fn potlock_accept_pot_rs() {
    world().run("scenarios/potlock-accept-pot.scen.json");
}

#[test]
fn potlock_add_pot_rs() {
    world().run("scenarios/potlock-add-pot.scen.json");
}

#[test]
fn potlock_apply_for_pot_rs() {
    world().run("scenarios/potlock-apply-for-pot.scen.json");
}

#[test]
fn potlock_deploy_rs() {
    world().run("scenarios/potlock-deploy.scen.json");
}

#[test]
fn potlock_distribute_pot_to_projects_rs() {
    world().run("scenarios/potlock-distribute-pot-to-projects.scen.json");
}

#[test]
fn potlock_distribute_to_project_less_than_max_percent_rs() {
    world().run("scenarios/potlock-distribute-to-project-less-than-max-percent.scen.json");
}

#[test]
fn potlock_donate_to_pot_rs() {
    world().run("scenarios/potlock-donate-to-pot.scen.json");
}

#[test]
fn potlock_donate_to_project_rs() {
    world().run("scenarios/potlock-donate-to-project.scen.json");
}

#[test]
fn potlock_fail_accept_application_rs() {
    world().run("scenarios/potlock-fail-accept-application.scen.json");
}

#[test]
fn potlock_fail_accept_pot_rs() {
    world().run("scenarios/potlock-fail-accept-pot.scen.json");
}

#[test]
fn potlock_fail_add_pot_rs() {
    world().run("scenarios/potlock-fail-add-pot.scen.json");
}

#[test]
fn potlock_fail_distribute_pot_to_projects_rs() {
    world().run("scenarios/potlock-fail-distribute-pot-to-projects.scen.json");
}

#[test]
fn potlock_fail_distribute_pot_to_projects_2_rs() {
    world().run("scenarios/potlock-fail-distribute-pot-to-projects2.scen.json");
}

#[test]
fn potlock_fail_donate_to_project_rs() {
    world().run("scenarios/potlock-fail-donate-to-project.scen.json");
}

#[test]
fn potlock_fail_remove_pot_rs() {
    world().run("scenarios/potlock-fail-remove-pot.scen.json");
}

#[test]
fn potlock_remove_pot_rs() {
    world().run("scenarios/potlock-remove-pot.scen.json");
}
