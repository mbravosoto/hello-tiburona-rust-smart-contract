# 🐋 Hello World Tiburonas – Soroban Smart Contract

A Rust-based Soroban smart contract that manages personalized greetings, user counters, and admin-controlled settings. Includes robust error handling, access control, and full test coverage.

## 🚀 Features

- 👋 Personalized greetings with name validation
- 🔢 Global and per-user greeting counters
- 🛡️ Admin-only access for sensitive operations
- ⚙️ Configurable character limit
- 🔄 Ownership transfer functionality
- 🧪 Comprehensive unit tests with panic handling


## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- `contacts/hello-world/lib.rs` is the main file which contains the contract and the tests.
- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.


## Prerequisites

Install Rustc, cargo and stellar-cli. Additional commands required for windows and to validate the installations:
* `rustc --version` >= 1.70
* `cargo --version` >= 1.70
* `stellar --version` >= 20.0.0
* `cargo install --locked stellar-cli`
* Add WebAssembly: `rustup target add wasm32-unknown-unknown`
* Enable Optimization Target: `rustup target add wasm32v1-none`


## Project Initialization

This step was already done to create a new project from scratch
`stellar contract init hello-tiburonas`


## Execution Procedure

* Validate the project: `cargo check`
[Cargo Check](img/cargo_check.png)
* Compile project and verify: `cargo build --target wasm32-unknown-unknown --release`. This command will generate.wasm file in `target/wasm32-unknown-unknown/release/hello_world.wasm`. `Validate with ls target/wasm32-unknown-unknown/release/`
[Cargo Build](img/cargo_build.png)
* Run tests: `cargo test`
[Cargo Tests](img/cargo_test.png)
* Optimized Build: `stellar contract build`. This command generates `hello_world.wasm` file
[Optimized Build](img/stellar_contract_build.png)
* Optimize WASM file: `stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello_world.wasm`
[Optimize WASM file](img/stellar_contract_optimize.png)


## 🚀 Contract Overview

The following functions will be found:

### Initialization

`initialize(env, admin_address)`
- Sets the admin
- Initializes global counter
- Sets default character limit (32)
- Extends TTL for storage

### Core Function

`hello(env, user, name) -> Result<Symbol, Error>`
- Validates name (non-empty, within limit)
- Increments global and per-user counters
- Stores last greeting
- Returns "Hola" as a symbol

### Consult Functions

- `get_contador(env) -> u32`: Returns total number of greetings
- `get_ultimo_saludo(env, user) -> Option<String>`: Returns the last greeting sent by a user
- `get_contador_usuario(env, user) -> u32`: Returns the number of greetings sent by a specific user

### 🔐 Admin Functions

- `reset_contador(env, caller) -> Result<(), Error>`: Resets the global greeting counter, only callable by admin
- `transfer_admin(env, caller, new_admin) -> Result<(), Error>`: Transfers contract ownership to a new admin
- `set_limite(env, caller, limit) -> Result<(), Error>`: Sets the maximum character limit for names

### ⚠️ Error Codes:

The error codes and function naming convention follow the traits to guarantee that this contact can communicate with others.
- **1**: NombreVacio - Name is empty
- **2**: NombreMuyLargo - Name exceeds character limit
- **3**: NoAutorizado - Caller is not authorized
- **4**: NoInicializado - Contract not initialized or already set


### 🗂️ Storage Strategy

- `instance()` → Global data (admin, counter, character limit)
- `persistent()` → Per-user data (last greeting, user counter)
- `TTL` extended by `100` blocks (min and max) to avoid indefinite storage


## 🧪 Unit Tests

The contract includes a comprehensive test suite using Soroban’s Env and Address test utilities. Run all tests with: `cargo test`

### ✅ Success Cases

- **test_initialize:** Initializes the contract and verifies the counter starts at 0
- **test_hello_exitoso:** Sends a valid greeting and checks counter and storage
- **test_reset_solo_admin:** Admin resets the counter successfully
- ⭐ **test_contador_usuario:** Verifies per-user counter retrieval from persistent storage

### ❌ Failure Cases

- **test_no_reinicializar:** Prevents reinitialization `(Error #4: NoInicializado)`
- **test_nombre_vacio:** Rejects empty name input `(Error #1: NombreVacio)`
- **test_reset_no_autorizado:** Blocks reset from non-admin user `(Error #3: NoAutorizado)`

### 🧪 Tests Highlights

- Uses `String::from_str(&env, "")` to simulate empty input
- Validates error codes with `#[should_panic(expected = "...")]`
- Accesses contract storage directly via `env.as_contract(...)` for setup


## 📚 Context

A basic hello-world Soroban contract can be generated using the CLI, but it typically lacks error handling, input validation, and access control. This project rewrites that foundation with a secure, production-ready implementation that includes robust logic, admin permissions, and test coverage.

To generate a simple hello-world contract:
`stellar contract init hello_world`

This creates a minimal contract structure inside the `contracts/hello_world/` directory.

To compile and run it with Rust:
`rustup target add wasm32-unknown-unknown`
`cargo build --target wasm32-unknown-unknown --release`

The compiled .wasm file will be located at:
`target/wasm32-unknown-unknown/release/hello_world.wasm`

Then can be deployed or optimized using Soroban CLI tools.


## Contributions
This project was made as part of Rust Advanced Soroban homework from [Código Futura course](https://github.com/BuenDia-Builders/codigofutura/tree/main/2da-semana-rust-consolidado/4-Clase) organized by Buen Día Builders.