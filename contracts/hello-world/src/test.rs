#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Logs, Env};

    #[test]
    fn test_add_product() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.add_product("ProductA".to_string(), 10, 100);
        let product = client.get_product("ProductA".to_string());
        assert_eq!(product, vec![&env, 10, 100]);
    }

    #[test]
    fn test_update_product() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.add_product("ProductA".to_string(), 10, 100);
        client.update_product("ProductA".to_string(), 20, 200);
        let product = client.get_product("ProductA".to_string());
        assert_eq!(product, vec![&env, 20, 200]);
    }

    #[test]
    fn test_delete_product() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.add_product("ProductA".to_string(), 10, 100);
        client.delete_product("ProductA".to_string());
        let product = client.get_product("ProductA".to_string());
        assert_eq!(product, vec![&env]);
    }

    #[test]
    fn test_get_all_products() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.add_product("ProductA".to_string(), 10, 100);
        client.add_product("ProductB".to_string(), 20, 200);
        let all_products = client.get_all_products();
        
        assert_eq!(all_products.len(), 2);
        assert_eq!(all_products[0], ("ProductA".to_string(), 10, 100));
        assert_eq!(all_products[1], ("ProductB".to_string(), 20, 200));
    }
}