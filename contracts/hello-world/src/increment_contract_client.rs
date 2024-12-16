// src/increment_contract_client.rs

use soroban_sdk::{Env, Symbol, Client};
use crate::increment_contract::IncrementContract;

pub struct IncrementContractClient {
    client: Client,
}

impl IncrementContractClient {
    // Crea una nueva instancia del cliente
    pub fn new(env: Env) -> Self {
        let client = Client::new(env);
        IncrementContractClient { client }
    }

    // Llama al método `increment` del contrato
    pub fn increment(&self, value: i32) -> i32 {
        let contract_id = self.client.contract_id();
        self.client.call::<i32, _>(contract_id, "increment", (value,))
    }

    // Llama al método `get_counter` para obtener el valor del contador
    pub fn get_counter(&self) -> i32 {
        let contract_id = self.client.contract_id();
        self.client.call::<i32, _>(contract_id, "get_counter", ())
    }
}
