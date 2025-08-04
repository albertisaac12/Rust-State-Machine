# Rust State Machine

This project implements a simple state machine in Rust, featuring a modular design with separate components for system and balance management. The project is structured to demonstrate the use of traits, generics, and modular programming in Rust.

## Project Structure

- **`main.rs`**: The entry point of the application. It initializes the runtime and demonstrates the usage of the `system` and `balances` modules.
- **`balances/`**: Contains the `balances` module, which manages account balances and provides functionality for balance transfers.
- **`system/`**: Contains the `system` module, which manages block numbers and account nonces.

## Modules

### `main.rs`
- Imports and initializes the `system` and `balances` modules.
- Defines the `Runtime` struct, which integrates the `system` and `balances` pallets.
- Implements configurations for the `system` and `balances` modules.
- Demonstrates the creation of a genesis state and basic operations.

### `balances` Module
- Manages account balances using a `BTreeMap`.
- Provides the following functionalities:
  - `set_balance`: Sets the balance for a specific account.
  - `get_balances`: Retrieves the balance of a specific account.
  - `transfer`: Transfers balance from one account to another, ensuring no underflow or overflow occurs.

### `system` Module
- Manages block numbers and account nonces using a `BTreeMap`.
- Provides the following functionalities:
  - `block_number`: Retrieves the current block number.
  - `inc_block_number`: Increments the block number.
  - `inc_nonce`: Increments the nonce for a specific account.

## Dependencies
- `num`: Used for numeric operations like `CheckedAdd` and `CheckedSub`.

## Usage
1. Clone the repository.
2. Build the project using Cargo:
   ```bash
   cargo build
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Example
The `main.rs` file demonstrates the following:
- Initializing the runtime.
- Setting a balance for an account.
- Performing a balance transfer.

## Future Enhancements
- Add more modules for additional functionalities.
- Implement comprehensive tests for all modules.
- Enhance error handling and logging.

## License
This project is licensed under the MIT License.