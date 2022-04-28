# Sample RabbitMq Consumer
This project presents a way to use RabbitMq as a message consumer with Rust and Actix Web

Technologies used: Rust, Actix Web, Serde, Tracing, Borsh, CrossTownBus

### Pre-Requires
- rust and cargo ([Install](https://www.rust-lang.org/tools/install))
- podman ([Install](https://podman.io/getting-started/installation))

### RabbitMq Producer
- producer ([Link](https://github.com/ronaldofjc/rust-actix-rabbit-producer))

### RabbitMq on Podman

- Run RabbitMq on Container

`podman run -d --hostname localhost --name local-rabbit -p 15672:15672 -p 5672:5672 rabbitmq:3-management`

- Stop container

`podman stop local-rabbit`

- Start container

`podman start local-rabbit`


### API Commands

- Compile project on develop

`cargo build`

- Compile on release

`cargo build --release`

- Execute on develop

`cargo watch -x run`

- Execute on release

`cargo run --release`






