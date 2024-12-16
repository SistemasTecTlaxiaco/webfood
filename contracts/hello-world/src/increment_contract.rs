// src/increment_contract.rs

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    // Método que incrementa un valor almacenado
    pub fn increment(env: Env, value: i32) -> i32 {
        // Almacenamos un valor en el almacenamiento persistente del contrato
        let mut counter: i32 = env.storage().get("counter").unwrap_or(0);
        counter += value;

        // Guardamos el nuevo valor del contador en el almacenamiento
        env.storage().set("counter", &counter);

        counter
    }

    // Método para obtener el valor actual del contador
    pub fn get_counter(env: Env) -> i32 {
        env.storage().get("counter").unwrap_or(0)
    }
}
