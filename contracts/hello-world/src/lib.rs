// ==== GENERAL DEFINITIONS ====
// Import:
// contracterror → Define errors
// contracttype → For DataKey
// Address → For access control
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String
};

// Standard errors to map the cause of the error to a number
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

// Define DataKey
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin, // Single global value
    ContadorSaludos,
    UltimoSaludo(Address), // Specific per user identified with the Address
    ContadorPorUsuario(Address),
    LimiteCaracteres,
}

// === CONTRACT ===
// MAIN MODULE: Define Contract
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // FUNCTIONS INSIDE CONTRACT
    
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        // Check if is already initialized
        // has() is cheaper, jst verifies existence without deserializing
        if env.storage().instance().has(&DataKey::Admin) {
            // NoInicializado means the operation (initialization) is not allowed as it was done before
            return Err(Error::NoInicializado);
        }
        // Save Admin
        env.storage()
            .instance() // Used for Admin as its unique and global
            .set(&DataKey::Admin, &admin);
        // Initialize counter
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32); // unsigned 32-bit with value 0

        // Extend TTL: How much time in blocks, data remain accesible in storage, once it expires data can be remmoved by the system
        // Guarantees that the data will leave at least 100 blocks more
        // Also guarantees that it doesn't extend indefinitely, COSTS GAS!!!
        env.storage()
            .instance()
            .extend_ttl(100, 100); // minimum_extension: u32, maximum_expiration: u32 - (At least 100 blocks more from current moment, Do not exceed 100 blocks from now)

        // Configure character limit
        env.storage()
            .instance()
            .set(&DataKey::LimiteCaracteres, &32u32);
        
        Ok(()) // Return Result value if OK, no need to return as is the last and no ;
    }

    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String
    ) -> Result<Symbol, Error> {
        // Validate ERRORS to avoid wasting gas storing the variables in the blockchain
        // Validate if the name is empty
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }

        // Get configurable character limit et by admin
        let limite: u32 = env.storage()
                            .instance()
                            .get(&DataKey::LimiteCaracteres)
                            .unwrap_or(32); // Use 32 as default if not configured

        // Validate if the name is too long
        if nombre.len() > limite {
            return Err(Error::NombreMuyLargo);
        }
        // Increase the counter
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador) // Read the value
            .unwrap_or(0); // Return 0 if unwrap to get the value fails
        
        // Modify the counter value and save
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1)); 

        // ⭐ Get and increase the counter per user
        let key_contador_usuario = DataKey::ContadorPorUsuario(usuario.clone());
        let contador_usuario = Self::get_contador_usuario(env.clone(), usuario.clone());
        env.storage()
            .persistent()
            .set(&key_contador_usuario, &(contador_usuario + 1)); 

        // Persist the last hello
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre); // Changes depending on the user that calls the function (personalized data) so needs to be persisted

        // Extend TTL
        env.storage()
            .persistent() // Persistent storage
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
        
        env.storage()
            .instance() // Instance storage
            .extend_ttl(100, 100);

        // Return hello
        Ok(Symbol::new(&env, "Hola"))
    }

    // CONSULT FUNCTIONS
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0) // Manages the error case assigning the value to zero, so always returns a number
    }
    
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> { // Option returns None if it doesn't exist
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }

    // ⭐ Bonus Function: Get Counter Per User
    pub fn get_contador_usuario(env: Env, usuario: Address) -> u32 {
        env.storage()
           .persistent()
           .get(&DataKey::ContadorPorUsuario(usuario))
           .unwrap_or(0) // Manages the error case assigning the value to zero, so always returns a number
    }

    // ADMIN FUNCTION
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        // Get admin address from instance storage
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin) 
            .ok_or(Error::NoInicializado)?; // If no admin address throws an Error NoInicializado and the function returns immediately

        // Validate permissions, just admin can reset counter
        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        // Reset counter
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32); // Assign value 0
        
        Ok(()) // Confirm success reseting the counters
    }

    // ⭐ Bonus Function: Tranfer ownership
    pub fn transfer_admin(
        env: Env,
        caller: Address,
        nuevo_admin: Address
    ) -> Result<(), Error> {
        // Get admin address from instance storage
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin) 
            .ok_or(Error::NoInicializado)?; // If no admin address throws an Error NoInicializado and the function returns immediately

        // Validate permissions, just admin can transfer ownership
        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        // Change the admin
        env.storage().instance().set(&DataKey::Admin, &nuevo_admin);

        Ok(()) // Confirm success changing the ownership
    }

    // ⭐ Bonus Function: Add configurable character limit
    pub fn set_limite(
        env: Env,
        caller: Address,
        limite: u32
    ) -> Result<(), Error> {
        // Get admin address from instance storage
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin) 
            .ok_or(Error::NoInicializado)?; // If no admin address throws an Error NoInicializado and the function returns immediately

        // Validate permissions, just admin can transfer ownership
        if caller != admin {
            return Err(Error::NoAutorizado);
        }

        // Save the new limit
        env.storage()
           .instance()
           .set(&DataKey::LimiteCaracteres, &limite);

        Ok(()) // Confirm success configuring character limit
    }
}

// TEST MODULE
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env,Address};
    use soroban_sdk::testutils::Address as TestAddress;

    // Successful initialization
    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        // First initialization must work
        client.initialize(&admin);
        
        // Validate counter 0
        assert_eq!(client.get_contador(), 0);
    }

    // Do NOT Reinitialize
    #[test]
    #[should_panic(expected = "Error(Contract, #4)")] // Returns NoInicializado = 4
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        client.initialize(&admin);
        client.initialize(&admin); // Seccond initialization will fail
    }

    // Successful hello
    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = String::from_str(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);
        
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }

    // Empty name fail
    #[test]
    #[should_panic(expected = "Error(Contract, #1)")] // Returns NombreVacio = 1
    fn test_nombre_vacio() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // ⭐ Usar String::from_str para string vacío
        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio);  // Debe fallar
    }

    // Reset just with admin permissions
    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // ⭐ Hacer saludos con String
        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        // Admin puede resetear
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    // Do not allow reset when no admin user
    #[test]
    #[should_panic(expected = "Error(Contract, #3)")] // Returns NoAutorizado = 3
    fn test_reset_no_autorizado() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        
        client.initialize(&admin);
        
        // If other user different than admin tries to reset
        client.reset_contador(&otro);  // Should fail
    }

    // ⭐ BONUS TESTS
    #[test]
    fn test_contador_usuario() {
        let env = Env::default();
        let usuario = Address::generate(&env);

        // Instance of the contract in env
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        // Use env as the contract to access the storage
        env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .set(&DataKey::ContadorPorUsuario(usuario.clone()), &3u32);
        });

        // Call th function as client of the contract
        let contador = client.get_contador_usuario(&usuario);
        assert_eq!(contador, 3);
    }
}