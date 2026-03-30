# Payments Engine

This is a simple payments engine that processes transactions from a CSV file and updates the account balances
accordingly. The output is exposed in a CSV format in the standard output.

## Usage

Run the following command to execute the payments engine:

```bash
cargo run -- test-transactions.csv > accounts.csv
```

## Tools

For the development of this project, I used the following tools:

- Git
- Rust + Cargo
- RustRover (IDE)
- Github Copilot (AI assistant): only for code suggestions and doc improvements, not for writing the code from scratch.

## Disputes and negative balances

As in real banking, disputes can cause the balance to go negative, if the client already spent part of the balance. This
means that if a client has a balance of 100€ and disputes a transaction of 150€, their balance will become -50€ until
the dispute is resolved.

I opted to allow negative balances in the system because it reflects the real-world scenario of disputes and their
impact on account balances.

## Deposits on locked accounts

When an account is locked, we should ignore any other operation but in some cases, like in crypto, someone can send you
funds to your account without your permission or any other kind of control. Therefore, even if the account is locked,
I decided to allow deposits to be processed, so this matches that real-world scenario.

## Past transactions

I store all past transactions in a map in memory for simplicity. In a real-world application, I would use a database to
store the transactions and account information, which would allow for better performance and scalability. Moreover, I
would include a TTL mechanism to remove old transactions that cannot be disputed anymore, which would help to keep the
database clean and efficient.

## Parallelism

This demo is designed to process transactions sequentially for simplicity. In a real-world application, I would consider
implementing parallel processing of transactions to improve performance, at least for reception, parsing and validation,
while keeping its processing sequential to avoid race conditions and ensure data consistency. More or less following the
same approach as Redis does with its single-threaded event loop.

## Testing

I have implemented unit tests for the core logic at [`state.rs`](src/state.rs) and manual testing against a test CSV
file that covers all cases contemplated in the requirements ([test-transactions.csv](test-transactions.csv)).

## Safety

### Error Handling

In this demo, I didn't implement any error handling for simplicity, as it is not defined any way of emitting them.
In a real-world application, I would implement proper error handling, logging system, etc.

### Dependencies

Normally I use only mayor version numbers for dependencies to keep them up-to-date. As this is a banking-related
project, it should be more stable and secured, so full version numbers are used to avoid any unexpected breaking
changes in the dependencies or unknown vulnerabilities.

### Quantities

For quantities, I use the `u64` file with a fixed number of decimal places (4 in this case) to represent the amounts.
This approach avoids issues with floating-point precision and ensures that all calculations are accurate and consistent.