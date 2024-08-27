use multiversx_sc_scenario::imports::*;
mod proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const SECOND_ADDRESS: TestAddress = TestAddress::new("second");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("update-attributes");
const CODE_PATH: MxscPath = MxscPath::new("output/update-attributes.mxsc.json");

struct UpdateAttributesState {
    world: ScenarioWorld,
}

impl UpdateAttributesState {
    fn new() -> Self {

        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);
        world.account(SECOND_ADDRESS).nonce(1);

        Self { world }
    }

    fn deploy_contract(&mut self){
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .init()
            .code(CODE_PATH)
            .new_address(SC_ADDRESS)
            .returns(ReturnsNewAddress)
            .run();
    }

    fn set_roles(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .set_roles()
            .run();
    }

    fn issue_fungible(&mut self) {
        let token_name = "Test Token";
        let tocken_ticker = "TT";
        let amount = 100u64;
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .issue_fungible(token_name,tocken_ticker, amount)
            .run();
    }

    fn issue_non_fungible(&mut self) {
        let token_name = "Test Token 2";
        let tocken_ticker = "TT2";
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .issue_non_fungible(token_name,tocken_ticker)
            .run();
    }

    fn issue_fungible_token_mapper(&mut self) {
        let token_name = "Test Token ";
        let tocken_ticker = "TT";
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .issue_fungible_token_mapper(token_name,tocken_ticker)
            .run();
    }

    fn create_nft(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .create_nft(OWNER_ADDRESS)
            .run();
    }

    fn send_nft(&mut self) {
        let token_nonce = 1u64;
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .send_nft(SECOND_ADDRESS, token_nonce)
            .run();
    }

    fn update_attributes(&mut self) {
        let attributes = "new attributes";
        let token_id = EgldOrEsdtTokenIdentifier::esdt("Test Token");
        let amount = BigUint::from(1u64);
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .update_attributes(attributes)
            .egld_or_single_esdt(&token_id, 0, &amount)
            .run();
    }

    fn check_balance_non_fungible(&mut self) {
        let token_id = TestTokenIdentifier::new("Test Token");
        self.world.check_account(OWNER_ADDRESS).esdt_balance(token_id, 0u64);
    }

    fn check_balance_fungible(&mut self) {
        let token_id = TestTokenIdentifier::new("Test Token 2");
        self.world.check_account(OWNER_ADDRESS).esdt_balance(token_id, 100u64);
    }
    
}

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, update_attributes::ContractBuilder);
    blockchain
}

#[test]
fn test_init() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
}

#[test]
fn test_issue_fungible() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
    world.issue_fungible();
    world.check_balance_fungible();
}

#[test]
fn test_issue_non_fungible() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
    world.issue_non_fungible();
    world.check_balance_non_fungible();
}

#[test]
fn test_issue_fungible_token_mapper() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
    world.issue_fungible_token_mapper();
}

#[test]
fn test_create_nft() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
    world.issue_non_fungible();
    world.create_nft();
}

#[test]
fn test_update_attributes() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
    world.issue_non_fungible();
    world.create_nft();
    world.update_attributes();
}

#[test]
fn test_send_nft() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.set_roles();
    world.issue_non_fungible();
    world.create_nft();
    world.send_nft();
}



