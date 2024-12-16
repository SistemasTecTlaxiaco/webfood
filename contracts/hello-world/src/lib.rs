#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, Vec, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Método para agregar un producto
    pub fn add_product(env: Env, name: String, quantity: i32, price: i32) {
        let key = symbol_short!("products");
        let mut products: Map<String, (i32, i32)> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

        // Si ya existe el producto, lo reemplazamos
        products.set(name.clone(), (quantity, price));
        env.storage().persistent().set(&key, &products);
    }

    // Método para obtener un producto
    pub fn get_product(env: Env, name: String) -> Option<(i32, i32)> {
        let key = symbol_short!("products");
        let products: Map<String, (i32, i32)> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));
        products.get(name) // Devuelve `None` si no se encuentra el producto
    }

    // Método para obtener todos los productos
    pub fn get_all_products(env: Env) -> Vec<(String, i32, i32)> {
        let key = symbol_short!("products");
        let products: Map<String, (i32, i32)> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));
        
        let mut result = Vec::new(&env);
        for (name, (quantity, price)) in products.iter() {
            result.push_back((name.clone(), quantity, price));
        }
        result
    }

    // Método para actualizar un producto
    pub fn update_product(env: Env, name: String, new_quantity: i32, new_price: i32) {
        let key = symbol_short!("products");
        let mut products: Map<String, (i32, i32)> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

        if products.contains_key(name.clone()) {
            products.set(name.clone(), (new_quantity, new_price));
            env.storage().persistent().set(&key, &products);
        }
    }

    // Método para eliminar un producto
    pub fn delete_product(env: Env, name: String) {
        let key = symbol_short!("products");
        let mut products: Map<String, (i32, i32)> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

        if products.contains_key(name.clone()) {
            products.remove(name.clone());
            env.storage().persistent().set(&key, &products);
        }
    }

    // Método ejemplo
    pub fn hello(env: Env, to: String) -> Vec<String> {
        let mut result = Vec::new(&env);
        result.push_back(String::from_str(&env, "Hello"));
        result.push_back(to);
        result
    }
}
