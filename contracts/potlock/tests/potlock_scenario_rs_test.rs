use multiversx_sc_scenario::*;
use multiversx_sc_scenario::imports::MxscPath;

const _: MxscPath = MxscPath::new("../output/potlock.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/potlock.mxsc.json", potlock::ContractBuilder);
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
