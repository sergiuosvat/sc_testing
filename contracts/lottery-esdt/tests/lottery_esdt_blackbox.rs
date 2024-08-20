use multiversx_sc_scenario::imports::*;

mod proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("OWNER_ADDRESS");
const SECOND_ADDRESS: TestAddress = TestAddress::new("SECOND_ADDRESS");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("lottery_esdt");
const CODE_PATH: MxscPath = MxscPath::new("../output/lottery-esdt.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    let token_id = TokenIdentifier::from_esdt_bytes(&b"BSK-476470"[..]); 

    blockchain.account(OWNER_ADDRESS).esdt_balance(&token_id, 1000);
    blockchain.account(SECOND_ADDRESS).esdt_balance(&token_id, 1000);

    blockchain.register_contract(CODE_PATH, lottery_esdt::ContractBuilder);
    blockchain
}

#[test]
fn lottery_esdt_blackbox_init(){
    let mut world = world();

    world.start_trace();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world.write_scenario_trace("scenarios/init-lottery-esdt.scen.json");
}

#[test]
fn lottery_esdt_blackbox_buy_all()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(SECOND_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ExpectError(4,"Lottery entry period has ended! Awaiting winner announcement."))
        .run();

    world.write_scenario_trace("scenarios/buy-all-tickets-and-exceed-max-tickets.scen.json");
}

#[test]
fn lottery_esdt_blackbox_buy_after_winner_announced()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(SECOND_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .determine_winner(&lottery_name)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ExpectError(4,"Lottery is currently inactive."))
        .run();

    world.write_scenario_trace("scenarios/buy-after-winner-announced.scen.json");
}

#[test]
fn lottery_esdt_blackbox_buy_after_deadline()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(SECOND_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    
    world.current_block().block_timestamp(30);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ExpectError(4,"Lottery entry period has ended! Awaiting winner announcement."))
        .run();

    world.write_scenario_trace("scenarios/buy-after-deadline.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_after_sold_out()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();
    world
        .tx()
        .from(SECOND_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ExpectError(4,"Lottery entry period has ended! Awaiting winner announcement."))
        .run();

    world.write_scenario_trace("scenarios/buy-after-sold-out.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_not_whitelisted()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(3u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(SECOND_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ExpectError(4,"You are not allowed to participate in this lottery!"))
        .run();

    world.write_scenario_trace("scenarios/buy-not-whitelisted.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_wrong_fee()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(3u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    let wrong_ticket_fee = BigUint::<StaticApi>::from(2u128);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&wrong_ticket_fee)
        .returns(ExpectError(4,"Wrong ticket fee!"))
        .run();

    world.write_scenario_trace("scenarios/buy-wrong-fee.scen.json");

}

#[test]
fn lottery_esdt_blackbox_determine_winner_early()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(3u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .determine_winner(&lottery_name)
        .returns(ExpectError(4,"Lottery is still running!"))
        .run();

    world.write_scenario_trace("scenarios/determine-winner-early.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_all_and_determine_winner()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(2u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution,
            opt_whitelist,
            opt_burn_percentage,
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .buy_ticket(&lottery_name)
        .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .determine_winner(&lottery_name)
        .returns(ReturnsResult)
        .run();

    world.write_scenario_trace("scenarios/buy-all-tickets-and-determine-winner.scen.json");

}

#[test]
fn lottery_esdt_blackbox_start_lottery_twice()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Lottery is already active!"))
        .run();

    world.write_scenario_trace("scenarios/start-lottery-twice.scen.json");

}

#[test]
fn lottery_esdt_blackbox_wrong_start_params()
{
    let mut world = world();

    world.start_trace();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::LotteryProxy)
        .init()
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();
    
    world.current_block().block_timestamp(10);
    
    let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
    let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
    let ticket_price = BigUint::<StaticApi>::from(1u128);
    let opt_total_tickets = Option::Some(2u32);
    let opt_deadline = Option::Some(20u64);
    let opt_max_entries_per_user = Option::Some(1u32);
    let prize_distribution_data: &[u8] = &[75,25];
    let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
    let mut whitelist = ManagedVec::new();
    whitelist.push(OWNER_ADDRESS.to_managed_address());
    whitelist.push(SECOND_ADDRESS.to_managed_address());
    let opt_whitelist = Option::Some(whitelist);
    let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;
    let opt_burn_percentage_some = OptionalValue::Some(BigUint::<StaticApi>::from(50u128));
    let token_id_burnable = TokenIdentifier::from_esdt_bytes(&b"TEST-123456"[..]);
    let boxed_bytes = token_id_burnable.to_boxed_bytes();
    world.set_esdt_local_roles(SC_ADDRESS, boxed_bytes.as_slice(), &[EsdtLocalRole::Burn]);

    let wrong_lottery_name = ManagedBuffer::new_from_bytes(&b""[..]);
    let wrong_token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b""[..]);
    let egld_token_identifier = EgldOrEsdtTokenIdentifier::egld();
    let wrong_ticket_price = BigUint::<StaticApi>::from(0u128);
    let wrong_total_tickets = Option::Some(0u32);
    let wrong_total_tickets2 = Option::Some(900u32);
    let wrong_deadline = Option::Some(0u64);
    let wrong_deadline2 = Option::Some(100000000u64);
    let wrong_max_entries_per_user = Option::Some(0u32);
    let wrong_prize_distribution_data :&[u8] = &[60,60];
    let wrong_prize_distribution = Option::Some(ManagedVec::from_iter(wrong_prize_distribution_data.iter().copied()));
    let worng_burn_percentage = OptionalValue::Some(BigUint::<StaticApi>::from(101u128));

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &wrong_lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4, "Name can't be empty!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &wrong_token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4, "Invalid token name provided!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &wrong_ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Ticket price must be higher than 0!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            wrong_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Must have more than 0 tickets available!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            wrong_total_tickets2,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Only 800 or less total tickets per lottery are allowed!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            wrong_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Deadline can't be in the past!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            wrong_deadline2,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Deadline can't be later than 30 days from now!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            wrong_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Must have more than 0 max entries per user!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            wrong_prize_distribution,
            opt_whitelist.clone(),
            opt_burn_percentage.clone(),
        )
        .returns(ExpectError(4,"Prize distribution must add up to exactly 100(%)!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &egld_token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage_some.clone(),
        )
        .returns(ExpectError(4,"EGLD can't be burned!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_identifier,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            opt_burn_percentage_some.clone(),
        )
        .returns(ExpectError(4,"The contract can't burn the selected token!"))
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::LotteryProxy)
        .start(
            &lottery_name,
            &token_id_burnable,
            &ticket_price,
            opt_total_tickets,
            opt_deadline,
            opt_max_entries_per_user,
            opt_prize_distribution.clone(),
            opt_whitelist.clone(),
            worng_burn_percentage,
        )
        .returns(ExpectError(4,"Invalid burn percentage!"))
        .run();

    world.write_scenario_trace("scenarios/wrong-start-params.scen.json");

}
