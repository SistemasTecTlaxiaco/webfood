#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_hello() {
    let env = Env::default();
    
    // Registra el contrato usando el método `register_contract`.
    let contract_id = env.register_contract(None, Contract);

    // Crea un cliente para interactuar con el contrato registrado.
    let client = ContractClient::new(&env, &contract_id);

    // Llama a la función `hello` del contrato.
    let result = client.hello(&String::from_str(&env, "Dev"));

    // Verifica que el resultado sea el esperado.
    assert_eq!(
        result,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Dev"),
        ]
    );
}
