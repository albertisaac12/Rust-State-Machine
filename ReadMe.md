# Rust State Machine

This project implements a simple state machine in Rust, featuring a modular design with separate components for system and balance management. The project is structured to demonstrate the use of traits, generics, and modular programming in Rust.

## Project Structure

- **`main.rs`**: The entry point of the application. It initializes the runtime and demonstrates the usage of the `system` and `balances` modules.
- **`balances/`**: Contains the `balances` module, which manages account balances and provides functionality for balance transfers.
- **`system/`**: Contains the `system` module, which manages block numbers and account nonces.

## Modules

### `main.rs`
The `main.rs` file serves as the entry point of the application. It integrates the `system` and `balances` modules into a unified runtime and demonstrates their usage.

#### Key Components:
- **`Runtime` Struct**: Combines the `system` and `balances` pallets.
  - **Generics**: The `Runtime` struct does not directly use generics but implements traits (`system::Config` and `balances::Config`) that define the types for `AccountId`, `BlockNumber`, `Nonce`, and `Balance`.
- **`execute_block` Function**: Processes a block by incrementing the block number, validating the block, and dispatching extrinsics.
  - **Generics**: Uses `types::Block`, which is defined as `Block<Header, Extrinsic>`. Here:
    - `Header` is `Header<BlockNumber>`.
    - `Extrinsic` is `Extrinsic<AccountId, RuntimeCall>`.

#### Important Terms:
- **`Extrinsic`**: Represents a transaction or operation in the blockchain.
- **`DispatchResult`**: Indicates the success or failure of a dispatched operation.

### `balances` Module
The `balances` module manages account balances and provides functionality for balance transfers.

#### Key Components:
- **`Pallet` Struct**: Stores balances in a `BTreeMap`.
  - **Generics**: `Pallet<T>`:
    - `T` is a type that implements the `Config` trait.
    - `T::AccountId` is the type for account identifiers.
    - `T::Balance` is the type for balances.
- **`set_balance` Function**: Sets the balance for a specific account.
  - **Generics**: Uses `T::AccountId` and `T::Balance`.
- **`transfer` Function**: Transfers balance between accounts, ensuring no underflow or overflow occurs.
  - **Generics**: Uses `T::AccountId` and `T::Balance`.

#### Important Terms:
- **`Call` Enum**: Represents the operations that can be performed, such as `Transfer`.
  - **Generics**: `Call<T>`:
    - `T` is a type that implements the `Config` trait.
    - `T::AccountId` and `T::Balance` are used in the `Transfer` variant.
- **`Dispatch` Trait**: Allows the module to handle calls and dispatch them appropriately.
  - **Generics**: `Dispatch` is implemented for `Pallet<T>`, where `T` implements `Config`.

### `system` Module
The `system` module manages block numbers and account nonces.

#### Key Components:
- **`Pallet` Struct**: Stores the current block number and account nonces.
  - **Generics**: `Pallet<T>`:
    - `T` is a type that implements the `Config` trait.
    - `T::BlockNumber` is the type for block numbers.
    - `T::Nonce` is the type for nonces.
    - `T::AccountId` is the type for account identifiers.
- **`inc_block_number` Function**: Increments the block number.
  - **Generics**: Uses `T::BlockNumber`.
- **`inc_nonce` Function**: Increments the nonce for a specific account.
  - **Generics**: Uses `T::AccountId` and `T::Nonce`.

#### Important Terms:
- **`Nonce`**: A unique number used to prevent replay attacks.
- **`BlockNumber`**: Represents the current block in the blockchain.

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