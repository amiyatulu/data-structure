## Ping tutorial

Here is a step-by-step tutorial on how to use libp2p to ping between two different networks in Rust:

**Prerequisites**

* You have Rust installed on your machine.
* You have two machines with different network connections (e.g., two separate computers, two virtual machines, or two containers).
* You have a basic understanding of libp2p and its concepts (e.g., nodes, peers, protocols).

**Step 1: Install libp2p**

Add the following line to your `Cargo.toml` file:
```toml
[dependencies]
libp2p = { version = "0.53.2", features = ["tcp", "tls", "dns", "async-std", "noise", "yamux", "websocket", "ping", "macros"] }
futures = "0.3.21"
async-std = { version = "1.12.0", features = ["attributes"] }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```
Then, run `cargo build` to download and compile libp2p.

**Step 2: Create a libp2p node on each machine**

Create a new Rust file (e.g., `main.rs`) on each machine with the following code:
```rust
use futures::prelude::*;
use libp2p::swarm::SwarmEvent;
use libp2p::{ping, Multiaddr};
use std::error::Error;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_async_std()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::tls::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(30))) // Allows us to observe pings for 30 seconds.
        .build();

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip6/::/tcp/0".parse()?)?;
    // Dial the peer identified by the multi-address given as the second
    // command-line argument, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}")
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}
```
This code creates a libp2p node with a random identity, a TCP transport, and a Ping protocol. It then starts the swarm and prints the node's multiaddr.

**Step 3: Run the nodes on each machine**

Run the code on each machine using `cargo run`. You should see the node's multiaddr printed on each machine.

**Step 4: Ping between nodes**

```bash
cargo run -p ping -- /ip6/2401:4900:3e92:...../tcp/44293
```

This code gets the swarm, parses the multiaddr of the other node, and pings the other node using the Ping protocol. The ping result will be printed to the console.

**That's it!** You should now have two libp2p nodes running on different networks, and you should be able to ping between them using the Ping protocol.

Note: Make sure to replace the `/ip6/2401:4900:3e92:...../tcp/44293` address with the actual multiaddr of the other node. Also, ensure that the networks are configured to allow traffic between the two machines.
