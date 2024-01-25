# HomeVision Challenge - Rust

## Steps to run the project

### Prerequisites

- Install Rust and Cargo (latest stable version).

### Setup

1. Create `photos` folder in the project's root directory.

### Run the project

1. Build the project

```
cargo build --release
```

2. Start application

```
cargo run --release <page> <per_page>
```

Example:

```
cargo run --release 1 10
```

3. Execution check
   - To check if application is working correctly, the user could check the `/photos` folder to see if photos have been downloaded. Additionally, the terminal will display some logs that describe the process.

## Solution

This Rust version of the HomeVision Challenge is a less robust but efficient demonstration of handling concurrent tasks. Unlike the Node.js version, this Rust implementation focuses on showcasing an alternative approach to concurrency, leveraging Rust's powerful multi-threading capabilities.

### Key points

- **Concurrency model in Rust**: Instead of using futures, which is conceptually similar to async-await in Node.js, this implementation utilizes tokio::spawn for managing concurrency. This approach aligns with Rust's strengths in handling parallel tasks efficiently.
- **Less overhead**: Initially, rayon was considered for its ease of use in handling parallel iterations. However, due to the potential overhead of starting and stopping processes a more direct approach with tokio was adopted. This approach was chosen considering the potential need to download significantly more than 10 photos simultaneously.
- **Less robust**: This version is straightforward, trying to show knowledge on the Rust's concurrency. It focuses on downloading photos concurrently without the additional complexity of features like retry mechanisms with backoff strategies or extensive error handling. It also does not include tests. It is primarily for demonstration purposes.

## Final remarks

This Rust version of the challenge serves as a complementary approach to the Node.js implementation, highlighting Rust's capabilities in handling concurrency and parallelism. It provides an insight into the trade-offs and design decisions unique to Rust, especially in the context of network I/O operations.
