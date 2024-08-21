use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn potlock_accept_application_go() {
    world().run("scenarios/potlock-accept-application.scen.json");
}

#[test]
fn potlock_accept_pot_go() {
    world().run("scenarios/potlock-accept-pot.scen.json");
}

#[test]
fn potlock_add_pot_go() {
    world().run("scenarios/potlock-add-pot.scen.json");
}

#[test]
fn potlock_apply_for_pot_go() {
    world().run("scenarios/potlock-apply-for-pot.scen.json");
}

#[test]
fn potlock_deploy_go() {
    world().run("scenarios/potlock-deploy.scen.json");
}

#[test]
fn potlock_distribute_pot_to_projects_go() {
    world().run("scenarios/potlock-distribute-pot-to-projects.scen.json");
}

#[test]
fn potlock_distribute_to_project_less_than_max_percent_go() {
    world().run("scenarios/potlock-distribute-to-project-less-than-max-percent.scen.json");
}

#[test]
fn potlock_donate_to_pot_go() {
    world().run("scenarios/potlock-donate-to-pot.scen.json");
}

#[test]
fn potlock_donate_to_pot_different_tokens_go() {
    world().run("scenarios/potlock-donate-to-pot-different-tokens.scen.json");
}

#[test]
fn potlock_donate_to_pot_same_token_go() {
    world().run("scenarios/potlock-donate-to-pot-same-token.scen.json");
}

#[test]
fn potlock_donate_to_project_go() {
    world().run("scenarios/potlock-donate-to-project.scen.json");
}

#[test]
fn potlock_fail_accept_application_go() {
    world().run("scenarios/potlock-fail-accept-application.scen.json");
}

#[test]
fn potlock_fail_accept_pot_go() {
    world().run("scenarios/potlock-fail-accept-pot.scen.json");
}

#[test]
fn potlock_fail_add_pot_go() {
    world().run("scenarios/potlock-fail-add-pot.scen.json");
}

#[test]
fn potlock_fail_distribute_pot_to_projects_go() {
    world().run("scenarios/potlock-fail-distribute-pot-to-projects.scen.json");
}

#[test]
fn potlock_fail_distribute_pot_to_projects_2_go() {
    world().run("scenarios/potlock-fail-distribute-pot-to-projects2.scen.json");
}

#[test]
fn potlock_fail_donate_to_project_go() {
    world().run("scenarios/potlock-fail-donate-to-project.scen.json");
}

#[test]
fn potlock_fail_remove_pot_go() {
    world().run("scenarios/potlock-fail-remove-pot.scen.json");
}

#[test]
fn potlock_multiple_change_fees_go() {
    world().run("scenarios/potlock-multiple-change-fees.scen.json");
}

#[test]
fn potlock_remove_pot_go() {
    world().run("scenarios/potlock-remove-pot.scen.json");
}
