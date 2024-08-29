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
            .create_nft()
            .run();
    }

    fn send_nft(&mut self) {
        let token_nonce = 1u64;
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .send_nft(SECOND_ADDRESS,token_nonce)
            .run();
    }

    fn update_attributes(&mut self, token_id: String) {
        let attributes = "new attributes";
        let token_id = TokenIdentifier::from_esdt_bytes(token_id.as_bytes());
        let amount = BigUint::from(1u64);
        self.world
            .tx()
            .from(SECOND_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .update_attributes(attributes)
            .single_esdt(&token_id, 1, &amount)
            .run();
    }

    fn test_get_nft_token_id(&mut self) -> String {
        let result = self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .nft_token_id()
            .returns(ReturnsResult)
            .run();
        result.to_string()
    }

    fn get_token_mapper(&mut self) -> String {
        let result = self.world
            .query()
            .to(SC_ADDRESS)
            .typed(proxy::UpdateAttributesProxy)
            .test_token_mapper()
            .returns(ReturnsResult)
            .run();
        result.to_string()
    }

    fn check_balance_non_fungible(&mut self) {
        let token_id = TestTokenIdentifier::new("Test Token");
        self.world.check_account(OWNER_ADDRESS).esdt_balance(token_id, 0u64);
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
    world.issue_fungible();
    world.issue_fungible_token_mapper();
    world.get_token_mapper();
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
    let token_id = world.test_get_nft_token_id();
    world.send_nft();
    world.update_attributes(token_id);
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

#[test]
fn test_double_non_fungible_issue() {
    let mut world = UpdateAttributesState::new();

    world.deploy_contract();
    world.issue_non_fungible();
    world.issue_non_fungible();
    world.set_roles();
}



