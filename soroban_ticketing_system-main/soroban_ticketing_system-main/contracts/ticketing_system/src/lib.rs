#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol, String};

// Define symbols for event and ticket counters
const EVENT_COUNTER: Symbol = symbol_short!("E_COUNTER");
const TICKET_COUNTER: Symbol = symbol_short!("T_COUNTER");

#[contract]
pub struct TicketingSystem;

#[contractimpl]
impl TicketingSystem {
    /// Initializes the contract by setting event and ticket counters to zero
    pub fn initialize(env: Env) {
        env.storage().instance().set(&EVENT_COUNTER, &0u32);
        env.storage().instance().set(&TICKET_COUNTER, &0u32);
    }

    /// Creates a new event and stores it in the contract
    pub fn create_event(env: Env, name: String, total_tickets: u32, ticket_price: u64) -> u32 {
        let mut event_count: u32 = env.storage().instance().get(&EVENT_COUNTER).unwrap_or(0);
        event_count += 1;
        env.storage().instance().set(&EVENT_COUNTER, &event_count);

        // let event_id = symbol_short!(&format!("EVENT_{}", event_count));
        env.storage().instance().set(&event_count, &(name, total_tickets, 0, ticket_price));

        log!(&env, "Created event {}: {} tickets at {} each", event_count, total_tickets, ticket_price);

        event_count
    }

    /// Sells a ticket for a specified event, updating the sold tickets count
    pub fn buy_ticket(env: Env, event_id: u32) -> u32 {
        // let event_symbol = symbol_short!(&format!("EVENT_{}", event_id));
        let (name, total_tickets, mut tickets_sold, ticket_price): (String, u32, u32, u64) =
            env.storage().instance().get(&event_id).unwrap();

        if tickets_sold >= total_tickets {
            panic!("All tickets are sold out");
        }

        tickets_sold += 1;
        env.storage().instance().set(&event_id, &(name.clone(), total_tickets, tickets_sold, ticket_price));

        let mut ticket_count: u32 = env.storage().instance().get(&TICKET_COUNTER).unwrap_or(0);
        ticket_count += 1;
        env.storage().instance().set(&TICKET_COUNTER, &ticket_count);

        // let ticket_symbol = symbol_short!(&format!("TICKET_{}", ticket_count));
        env.storage().instance().set(&event_id,&env.current_contract_address());

        log!(&env, "Sold ticket {}: Event {} to {}", ticket_count, name.clone(), &env.current_contract_address());

        ticket_count
    }

    /// Checks the ownership of a specific ticket
    pub fn check_ticket_owner(env: Env, ticket_id: u32) -> String {
        // let ticket_symbol = symbol_short!(&format!("TICKET_{}", ticket_id));
        let owner: String = env.storage().instance().get(&ticket_id).unwrap();
        log!(&env, "Ticket {} is owned by {}", ticket_id, owner);
        owner
    }

    pub fn get_event_data(env: Env, event_id: u32) -> (String, u32, u32, u64) {
        env.storage().instance().get(&event_id).unwrap()
    }
}

mod test;