# B (is for blockchain)
## About
A basic blockchain emulator writen in Rust. Provides a server and client in one CLI application. When server is running, all other commands will interact with the server. If the server is not running, the commands will fail. 

## Usage

This CLI application provides several commands to manage and interact with accounts and a node server. The commands include `start-node`, `create-account`, `transfer`, and `balance`. You can also control the verbosity of logging using the `-v` flag.

### Command Line Arguments
All commands can be pre-empted with the following flags:

- `-p`, `--port`: The port number for connecting or listening (default: 9999).
- `-v`: Verbose mode. Sets the log level to debug.
- `-i`: The interval (in seconds) for the start-node command.

### Commands

1. **start-node**
    - Starts the node server and listens for connections.
    - **Usage**: 
      ```sh
      b start-node
      ```
    - **Example**:
      ```sh
      b start-node
      ```

2. **create-account**
    - Creates a new account with a specified ID and starting balance.
    - **Usage**: 
      ```sh
      b create-account <id-of-account> <starting-balance>
      ```
    - **Example**:
      ```sh
      b create-account 12345 1000.0
      ```

3. **transfer**
    - Transfers a specified amount from one account to another.
    - **Usage**: 
      ```sh
      b transfer <from-account> <to-account> <amount>
      ```
    - **Example**:
      ```sh
      b transfer 12345 67890 250.0
      ```

4. **balance**
    - Checks the balance of an account.
    - **Usage**: 
      ```sh
      b balance
      ```
    - **Example**:
      ```sh
      b balance
      ```
