use multiversx_sc_scenario::imports::*;

mod proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("OWNER_ADDRESS");
const BUYER_ADDRESS: TestAddress = TestAddress::new("BUYER_ADDRESS");
const TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("TOKEN_IDENTIFIER");
const WANTED_NFT: TestTokenIdentifier = TestTokenIdentifier::new("WANTED_NFT");
const DIFFERENT_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("DIFFERENT_TOKEN");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("SC_ADDRESS");


const CODE_PATH: MxscPath = MxscPath::new("../output/nft-escrow.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, nft_escrow::ContractBuilder);
    blockchain
}

struct NftEscrowTestState {
    world: ScenarioWorld,
}

impl NftEscrowTestState {
    fn new(world: ScenarioWorld) -> Self {
        
        let mut world = world;

        world
            .account(OWNER_ADDRESS)
            .esdt_nft_balance(TOKEN_IDENTIFIER, 1, 10, ManagedBuffer::new())
            .esdt_nft_balance(TOKEN_IDENTIFIER, 0, 10, ManagedBuffer::new());

        world
        .account(BUYER_ADDRESS)
        .esdt_nft_balance(WANTED_NFT, 1, 10,ManagedBuffer::new())
        .esdt_nft_balance(WANTED_NFT, 2, 10, ManagedBuffer::new())
        .esdt_nft_balance(DIFFERENT_TOKEN, 1, 10, ManagedBuffer::new());
        
        Self { world }
    }

    fn deploy(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(proxy::NftEscrowContractProxy)
            .init()
            .code(CODE_PATH)
            .code_metadata(CodeMetadata::PAYABLE)
            .returns(ReturnsNewAddress)
            .new_address(SC_ADDRESS)
            .run();
    }

    fn escrow(&mut self) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::NftEscrowContractProxy)
            .escrow(WANTED_NFT, 1u64, BUYER_ADDRESS)
            .egld_or_single_esdt(
                &EgldOrEsdtTokenIdentifier::esdt(TOKEN_IDENTIFIER),
                1u64,
                &BigUint::from(1u64),
            )
            .returns(ReturnsResult)
            .run();
    }

    fn accept_offer(&mut self, offer_id: u32) {
        self.world
            .tx()
            .from(BUYER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::NftEscrowContractProxy)
            .accept(offer_id)
            .egld_or_single_esdt(&EgldOrEsdtTokenIdentifier::esdt(WANTED_NFT), 1u64, &BigUint::from(1u64))
            .returns(ReturnsResult)
            .run();
    }

    fn cancel_offer(&mut self, offer_id: u32) {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::NftEscrowContractProxy)
            .cancel(offer_id)
            .returns(ReturnsResult)
            .run();
    }

    fn get_created_offers(&mut self, address: TestAddress) -> usize {
        let result = self.world
            .tx()
            .from(address)
            .to(SC_ADDRESS)
            .typed(proxy::NftEscrowContractProxy)
            .get_created_offers(address)
            .returns(ReturnsResult)
            .run();

        result.len()
    }

    fn get_wanted_offers(&mut self, address: TestAddress) -> usize {
        let result = self.world
            .tx()
            .from(address)
            .to(SC_ADDRESS)
            .typed(proxy::NftEscrowContractProxy)
            .get_wanted_offers(address)
            .returns(ReturnsResult)
            .run();

        result.len()
    }

}

#[test]
fn test_deploy() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
}

#[test]
fn test_escrow_success() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();

    state.escrow();
}

#[test]
fn test_escrow_success_twice() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();

    state.escrow();
    state.escrow();
}

#[test]
fn test_accept_offer_success() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();

    state.escrow();
    state.accept_offer(1u32);
}

#[test]
fn test_cancel_offer_success() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();

    state.escrow();
    state.cancel_offer(1u32);
}

#[test]
fn test_multiple_offers_interaction(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();
    state.escrow();
    state.accept_offer(1u32);
    state.cancel_offer(2u32);
}

#[test]
fn test_get_created_offers(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    let len = state.get_created_offers(OWNER_ADDRESS);
    assert_eq!(len, 0);
}

#[test]
fn test_get_wanted_offers(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    let len = state.get_wanted_offers(BUYER_ADDRESS);
    assert_eq!(len, 0);
}

//Negative tests

#[test]
fn test_fail_escrow_nonce_zero() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .escrow(WANTED_NFT, 1u64, BUYER_ADDRESS)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(TOKEN_IDENTIFIER),
            0u64,
            &BigUint::from(1u64),
        )
        .returns(ExpectError(4, "ESDT is not an NFT"))
        .run();
}

#[test]
fn test_fail_escrow_token_payment_value_not_1() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .escrow(WANTED_NFT, 1u64, BUYER_ADDRESS)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(TOKEN_IDENTIFIER),
            1u64,
            &BigUint::from(2u64),
        )
        .returns(ExpectError(4, "ESDT is not an NFT"))
        .run();
}

#[test]
fn test_fail_escrow_wanted_token_nonce_0() {
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .escrow(WANTED_NFT, 0u64, BUYER_ADDRESS)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(TOKEN_IDENTIFIER),
            1u64,
            &BigUint::from(1u64),
        )
        .returns(ExpectError(4, "Wanted ESDT is not an NFT"))
        .run();
}

#[test]
fn test_fail_escrow_same_address(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .escrow(WANTED_NFT, 1u64, OWNER_ADDRESS)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(TOKEN_IDENTIFIER),
            1u64,
            &BigUint::from(1u64),
        )
        .returns(ExpectError(4, "Wanted address should not be the same as the caller"))
        .run();
}

#[test]
fn test_fail_accept_offer_non_existent()
{
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state
        .world
        .tx()
        .from(BUYER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .accept(1u32)
        .returns(ExpectError(4, "Offer does not exist"))
        .run();
}

#[test]
fn test_fail_accept_unauthorized_acceptance()
{
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();

    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .accept(1u32)
        .returns(ExpectError(4, "Can not accept this offer"))
        .run();
}

#[test]
fn test_fail_accept_token_amount_not_1()
{
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();

    state
        .world
        .tx()
        .from(BUYER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .accept(1u32)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(WANTED_NFT),
            1u64,
            &BigUint::from(2u64),
        )
        .returns(ExpectError(4, "NFT does not match"))
        .run();
}

#[test]
fn test_fail_accept_nft_does_not_match_nonce(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();

    state
        .world
        .tx()
        .from(BUYER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .accept(1u32)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(WANTED_NFT),
            2u64,
            &BigUint::from(1u64),
        )
        .returns(ExpectError(4, "NFT does not match"))
        .run();
}

#[test]
fn test_fail_nft_does_not_match_token(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();

    state
        .world
        .tx()
        .from(BUYER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .accept(1u32)
        .egld_or_single_esdt(
            &EgldOrEsdtTokenIdentifier::esdt(DIFFERENT_TOKEN),
            1u64,
            &BigUint::from(1u64),
        )
        .returns(ExpectError(4, "NFT does not match"))
        .run();
}

#[test]
fn test_cancel_offer_non_existent(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state
        .world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .cancel(1u32)
        .returns(ExpectError(4, "Offer does not exist"))
        .run();
}

#[test]
fn test_cancel_offer_different_address(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();

    state
        .world
        .tx()
        .from(BUYER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .cancel(1u32)
        .returns(ExpectError(4, "Only the offer creator can cancel it"))
        .run();
}

#[test]
fn test_fail_accept_offer_twice(){
    let mut state = NftEscrowTestState::new(world());
    state.deploy();
    
    state.escrow();
    state.accept_offer(1u32);

    state
        .world
        .tx()
        .from(BUYER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::NftEscrowContractProxy)
        .accept(1u32)
        .returns(ExpectError(4, "Offer does not exist"))
        .run();
}
