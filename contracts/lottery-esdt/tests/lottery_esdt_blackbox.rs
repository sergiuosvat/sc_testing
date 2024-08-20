use multiversx_sc_scenario::imports::*;

mod proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("OWNER_ADDRESS");
const FIRST_ADDRESS: TestAddress = TestAddress::new("FIRST_ADDRESS");
const SECOND_ADDRESS: TestAddress = TestAddress::new("SECOND_ADDRESS");
const THIRD_ADDRESS: TestAddress = TestAddress::new("THIRD_ADDRESS");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("lottery-esdt");
const CODE_PATH: MxscPath = MxscPath::new("../output/lottery-esdt.mxsc.json");
const TOKEN_IDENTIFIER: TestTokenIdentifier = TestTokenIdentifier::new("BSK-476470");
const TOKEN_BURNABLE: TestTokenIdentifier = TestTokenIdentifier::new("TEST-123456");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, lottery_esdt::ContractBuilder);
    blockchain
}

struct LotteryESDTTestState {
    world: ScenarioWorld,
}

impl LotteryESDTTestState{
    fn new() -> Self {
        let mut world = world();

        world.account(OWNER_ADDRESS).nonce(1);

        world
            .account(FIRST_ADDRESS)
            .esdt_balance(TOKEN_IDENTIFIER, 1000)
            .nonce(1);

        world
            .account(SECOND_ADDRESS)
            .esdt_balance(TOKEN_IDENTIFIER, 1000)
            .nonce(1);

        world  
            .account(THIRD_ADDRESS)
            .esdt_balance(TOKEN_IDENTIFIER, 1000)
            .nonce(1);

        

        world.current_block().block_timestamp(10);

        Self { world }
    }

    fn start_trace(&mut self){
        self.world.start_trace();
    }

    fn deploy(&mut self){
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(proxy::LotteryProxy)
            .init()
            .code(CODE_PATH)
            .code_metadata(CodeMetadata::PAYABLE)
            .returns(ReturnsNewAddress)
            .new_address(SC_ADDRESS)
            .run();
    }

    fn write_scenario_trace(&mut self, file_name: &str){
        self.world.write_scenario_trace(file_name);
    }

    fn start_lottery(&mut self)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
        let ticket_price = BigUint::<StaticApi>::from(1u128);
        let opt_total_tickets = Option::Some(2u32);
        let opt_deadline = Option::Some(20u64);
        let opt_max_entries_per_user = Option::Some(1u32);
        let prize_distribution_data: &[u8] = &[75,25];
        let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
        let mut whitelist = ManagedVec::new();
        whitelist.push(FIRST_ADDRESS.to_managed_address());
        whitelist.push(SECOND_ADDRESS.to_managed_address());
        let opt_whitelist = Option::Some(whitelist);
        let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

        self.world
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
    }

    fn start_lottery_error(&mut self, error: ExpectError)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
        let ticket_price = BigUint::<StaticApi>::from(1u128);
        let opt_total_tickets = Option::Some(2u32);
        let opt_deadline = Option::Some(20u64);
        let opt_max_entries_per_user = Option::Some(1u32);
        let prize_distribution_data: &[u8] = &[75,25];
        let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));
        let mut whitelist = ManagedVec::new();
        whitelist.push(FIRST_ADDRESS.to_managed_address());
        whitelist.push(SECOND_ADDRESS.to_managed_address());
        let opt_whitelist = Option::Some(whitelist);
        let opt_burn_percentage: OptionalValue<BigUint<StaticApi>> = OptionalValue::None;

        self.world
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
            .returns(error)
            .run();
    }

    fn start_lottery_error_params(&mut self, lottery_name_wrong: bool, token_identifier_wrong:u64, ticket_price_wrong:bool, opt_total_tickets_wrong: Option<u32>, opt_deadline_wrong: Option<u64>,
         opt_max_entries_per_user_wrong: bool, opt_prize_distribution_wrong: bool, opt_burn_percentage_wrong: bool, error: ExpectError)
    {
        let lottery_name = if lottery_name_wrong {
            ManagedBuffer::new_from_bytes(&b""[..])
        } else {
            ManagedBuffer::new_from_bytes(&b"test"[..])
        };

        let ticket_price = if ticket_price_wrong {
            BigUint::<StaticApi>::from(0u128)
        } else {
            BigUint::<StaticApi>::from(1u128)
        };

        let opt_max_entries_per_user = if opt_max_entries_per_user_wrong {
            Option::Some(0u32)
        } else {
            Option::Some(1u32)
        };

        let prize_distribution_data: &[u8] = if opt_prize_distribution_wrong {
            &[60,60]
        } else {
            &[75,25]
        };

        let token_identifier;

        match token_identifier_wrong{
            1 => token_identifier = TestTokenIdentifier::new(""),
            2 => token_identifier = TestTokenIdentifier::new("EGLD"),
            3 => token_identifier = TOKEN_BURNABLE,
            _ => token_identifier = TOKEN_IDENTIFIER
        }

        let opt_prize_distribution = Option::Some(ManagedVec::from_iter(prize_distribution_data.iter().copied()));

        let mut whitelist = ManagedVec::new();
        whitelist.push(FIRST_ADDRESS.to_managed_address());
        whitelist.push(SECOND_ADDRESS.to_managed_address());

        let opt_whitelist = Option::Some(whitelist);

        let opt_burn_percentage = if opt_burn_percentage_wrong {
            OptionalValue::Some(BigUint::<StaticApi>::from(101u128))
        } else {
            OptionalValue::Some(BigUint::<StaticApi>::from(10u128))
        };
        
        const TOKEN_ID_BURNABLE: &[u8] = b"TEST-123456";
        self.world.set_esdt_local_roles(SC_ADDRESS, TOKEN_ID_BURNABLE, &[EsdtLocalRole::Burn]);

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::LotteryProxy)
            .start(
                &lottery_name,
                &token_identifier,
                &ticket_price,
                opt_total_tickets_wrong,
                opt_deadline_wrong,
                opt_max_entries_per_user,
                opt_prize_distribution,
                opt_whitelist,
                opt_burn_percentage,
            )
            .returns(error)
            .run();
    }

    fn buy_ticket(&mut self, address: TestAddress)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
        let ticket_price = BigUint::<StaticApi>::from(1u128);

        self.world
            .tx()
            .from(address)
            .to(SC_ADDRESS)
            .typed(proxy::LotteryProxy)
            .buy_ticket(&lottery_name)
            .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
            .returns(ReturnsResult)
            .run();
    }

    fn buy_ticket_error(&mut self, address: TestAddress, error: ExpectError)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);
        let ticket_price = BigUint::<StaticApi>::from(1u128);

        self.world
            .tx()
            .from(address)
            .to(SC_ADDRESS)
            .typed(proxy::LotteryProxy)
            .buy_ticket(&lottery_name)
            .egld_or_single_esdt(&token_identifier, 0,&ticket_price)
            .returns(error)
            .run();
    }

    fn buy_ticket_wrong_fee(&mut self, address: TestAddress, fee: BigUint<StaticApi>)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);
        let token_identifier = EgldOrEsdtTokenIdentifier::esdt(&b"BSK-476470"[..]);

        self.world
            .tx()
            .from(address)
            .to(SC_ADDRESS)
            .typed(proxy::LotteryProxy)
            .buy_ticket(&lottery_name)
            .egld_or_single_esdt(&token_identifier, 0,&fee)
            .returns(ExpectError(4,"Wrong ticket fee!"))
            .run();
    }

    fn determine_winner(&mut self)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::LotteryProxy)
            .determine_winner(&lottery_name)
            .returns(ReturnsResult)
            .run();
    }

    fn determine_winner_error(&mut self, error: ExpectError)
    {
        let lottery_name = ManagedBuffer::new_from_bytes(&b"test"[..]);

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .typed(proxy::LotteryProxy)
            .determine_winner(&lottery_name)
            .returns(error)
            .run();
    }

    fn set_time_block(&mut self, timestamp: u64){
        self.world.current_block().block_timestamp(timestamp);
    }

}

#[test]
fn lottery_esdt_blackbox_init(){
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.write_scenario_trace("scenarios/init-lottery-esdt.scen.json");
}

#[test]
fn lottery_esdt_blackbox_buy_all()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();
    
    world.start_lottery();

    world.buy_ticket(FIRST_ADDRESS);

    world.buy_ticket(SECOND_ADDRESS);

    world.buy_ticket_error(FIRST_ADDRESS, ExpectError(4,"Lottery entry period has ended! Awaiting winner announcement."));

    world.write_scenario_trace("scenarios/buy-all-tickets-and-exceed-max-tickets.scen.json");
}

#[test]
fn lottery_esdt_blackbox_buy_after_winner_announced()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();
    
    world.buy_ticket(FIRST_ADDRESS);

    world.buy_ticket(SECOND_ADDRESS);

    world.determine_winner();

    world.buy_ticket_error(FIRST_ADDRESS, ExpectError(4,"Lottery is currently inactive."));
    
    world.write_scenario_trace("scenarios/buy-after-winner-announced.scen.json");
}

#[test]
fn lottery_esdt_blackbox_buy_after_deadline()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();

    world.buy_ticket(FIRST_ADDRESS);

    world.buy_ticket(SECOND_ADDRESS);

    world.set_time_block(30);
    
    world.buy_ticket_error(FIRST_ADDRESS, ExpectError(4,"Lottery entry period has ended! Awaiting winner announcement."));

    world.write_scenario_trace("scenarios/buy-after-deadline.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_after_sold_out()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();
    
    world.buy_ticket(FIRST_ADDRESS);

    world.buy_ticket(SECOND_ADDRESS);

    world.buy_ticket_error(FIRST_ADDRESS, ExpectError(4,"Lottery entry period has ended! Awaiting winner announcement."));

    world.write_scenario_trace("scenarios/buy-after-sold-out.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_not_whitelisted()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();
    
    world.buy_ticket_error(THIRD_ADDRESS, ExpectError(4, "You are not allowed to participate in this lottery!"));

    world.write_scenario_trace("scenarios/buy-not-whitelisted.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_wrong_fee()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();
    
    world.buy_ticket_wrong_fee(FIRST_ADDRESS, BigUint::<StaticApi>::from(2u128));
    
    world.write_scenario_trace("scenarios/buy-wrong-fee.scen.json");

}

#[test]
fn lottery_esdt_blackbox_determine_winner_early()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();
    
    world.start_lottery();
    
    world.buy_ticket(FIRST_ADDRESS);

    world.determine_winner_error(ExpectError(4,"Lottery is still running!"));

    world.write_scenario_trace("scenarios/determine-winner-early.scen.json");

}

#[test]
fn lottery_esdt_blackbox_buy_all_and_determine_winner()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();

    world.buy_ticket(FIRST_ADDRESS);

    world.buy_ticket(SECOND_ADDRESS);

    world.determine_winner();

    world.write_scenario_trace("scenarios/buy-all-tickets-and-determine-winner.scen.json");

}

#[test]
fn lottery_esdt_blackbox_start_lottery_twice()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();

    world.start_lottery();

    world.start_lottery_error(ExpectError(4,"Lottery is already active!"));

    world.write_scenario_trace("scenarios/start-lottery-twice.scen.json");

}

#[test]
fn lottery_esdt_blackbox_wrong_start_params()
{
    let mut world = LotteryESDTTestState::new();

    world.start_trace();

    world.deploy();
    
    let wrong_total_tickets = Option::Some(0u32);
    let wrong_total_tickets2 = Option::Some(900u32);
    let wrong_deadline = Option::Some(0u64);
    let wrong_deadline2 = Option::Some(100000000u64);
    let total_tickets = Option::Some(2u32);
    let deadline = Option::Some(20u64);

    world.start_lottery_error_params(true, 0, false, total_tickets, deadline, false, false, false, ExpectError(4,"Name can't be empty!"));

    world.start_lottery_error_params(false, 1, false, total_tickets, deadline, false, false, false, ExpectError(4,"Invalid token name provided!"));

    world.start_lottery_error_params(false, 0, true, total_tickets, deadline, false, false, false, ExpectError(4,"Ticket price must be higher than 0!"));

    world.start_lottery_error_params(false, 0, false, wrong_total_tickets, deadline, false, false, false, ExpectError(4,"Must have more than 0 tickets available!"));

    world.start_lottery_error_params(false, 0, false, wrong_total_tickets2, deadline, false, false, false, ExpectError(4,"Only 800 or less total tickets per lottery are allowed!"));

    world.start_lottery_error_params(false, 0, false, total_tickets, wrong_deadline, false, false, false, ExpectError(4,"Deadline can't be in the past!"));

    world.start_lottery_error_params(false, 0, false, total_tickets, wrong_deadline2, false, false, false, ExpectError(4,"Deadline can't be later than 30 days from now!"));

    world.start_lottery_error_params(false, 0, false,total_tickets, deadline, true, false, false, ExpectError(4,"Must have more than 0 max entries per user!"));

    world.start_lottery_error_params(false, 0, false, total_tickets, deadline, false, true, false, ExpectError(4,"Prize distribution must add up to exactly 100(%)!"));

    world.start_lottery_error_params(false, 2, false, total_tickets, deadline, false, false, true, ExpectError(4,"EGLD can't be burned!"));

    world.start_lottery_error_params(false, 0, false, total_tickets, deadline, false, false, true, ExpectError(4,"The contract can't burn the selected token!"));

    world.start_lottery_error_params(false, 3, false, total_tickets, deadline, false, false, true, ExpectError(4,"Invalid burn percentage!"));

    world.write_scenario_trace("scenarios/wrong-start-params.scen.json");

}
