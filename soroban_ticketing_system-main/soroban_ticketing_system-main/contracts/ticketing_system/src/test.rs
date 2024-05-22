#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env,String};

// Use wee_alloc as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[test]
fn test_initialization() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicketingSystem);
    let client = TicketingSystemClient::new(&env, &contract_id);

    client.initialize();

    let event_counter: u32 = env.storage().instance().get(&symbol_short!("E_COUNTER")).unwrap_or(0);
    let ticket_counter: u32 = env.storage().instance().get(&symbol_short!("T_COUNTER")).unwrap_or(0);
    assert_eq!(event_counter, 0);
    assert_eq!(ticket_counter, 0);
}

#[test]
fn test_create_event() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicketingSystem);
    let client = TicketingSystemClient::new(&env, &contract_id);

    client.initialize();
    
    // Define the parameters
    // let name = String::From("Concert");
    let rust_string = "Concert";
let soroban_string = soroban_sdk::String::from_str(&env, rust_string);
    let total_tickets = 100;
    let ticket_price = 50u64;

    // Pass references to the parameters
    let event_id = client.create_event(&soroban_string, &total_tickets, &ticket_price);

    assert_eq!(event_id, 1);

    // Assume there's a method to retrieve event data
    let event_data: (String, u32, u32, u64) = client.get_event_data(&event_id);

    let first_val:u32 = 100;
    let second_val:u32 = 0;
    let third_val:u64 = 50;


    assert_eq!(event_data, (soroban_string, first_val, second_val, third_val));
}

#[test]
fn test_buy_ticket() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicketingSystem);
    let client = TicketingSystemClient::new(&env, &contract_id);

    client.initialize();

    let total_tickets = 100; // u32
let ticket_price = 50;   // u64

    let rust_string = "Concert";
    let soroban_string = soroban_sdk::String::from_str(&env, rust_string);
    let event_id = client.create_event(&soroban_string, &total_tickets, &ticket_price);
    let ticket_id = client.buy_ticket(&event_id);

    assert_eq!(ticket_id, 1); // Assuming ticket IDs start at 1 and increment

    let (name, total_tickets, tickets_sold, price): (String, u32, u32, u64) = env.storage().instance().get(&event_id).unwrap();
    assert_eq!(tickets_sold, 1);
}

#[test]
#[should_panic(expected = "All tickets are sold out")]
fn test_sold_out_event() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TicketingSystem);
    let client = TicketingSystemClient::new(&env, &contract_id);

    client.initialize();

    let rust_string = "Concert";
    let soroban_string = soroban_sdk::String::from_str(&env, rust_string);
    
    let total_tickets = 1; // u32
    let ticket_price = 100;  

    let event_id = client.create_event(&soroban_string, &total_tickets, &ticket_price); // Only 1 ticket available

    let _ = client.buy_ticket(&event_id); // This should work
    let _ = client.buy_ticket(&event_id); // This should panic
}
