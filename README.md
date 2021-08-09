# Rustycraft

For the original project, go to https://github.com/feather-rs/feather

A Minecraft server implementation written in Rust.

### Features

Note that Feather is still early in development. Don't expect anything not listed here to work.

- [x] Highly scalable architecture
- [x] Anvil world loading and saving
- [x] Physics
- [x] Basic world generation
- [x] Chunk streaming
- [x] Day/night cycle
- [x] Weather
- [x] Block lighting
- [x] Arrow shooting
- [x] Falling blocks
- [x] Block placement and breaking
- [x] Item dropping and collection
- [x] Chat
- [x] Inventory handling
- [x] Movement broadcasting
- [x] Commands (/tell, /tp, /gamemode)
- [x] Survival mode
- [x] Survival mode block breaking and drops
- [x] Health + fall damage

### Planned Features

- [ ] block entities, including chests
- [ ] HP regen
- [ ] Mob spawning
- [ ] Hunger
- [ ] Food
- [ ] Save world and player position + items

Rustycraft currently only supports 1.13.2 clients and world saves. In the future, additional versions will be supported.

### Compiling
Compile the server yourself to try it out:
```bash
git clone https://github.com/feather-rs/feather
cd feather
cargo build --release
```

Compiling from source requires `rustfmt`.

The server executable will be located in `target/release`.

### FAQ

* Is Rustycraft production ready?

Not yet. There are numerous bugs and missing features which have yet to be resolved,
and the codebase has not been tested enough to consider the server production ready.

* Where is the original project?

It's located [here](https://github.com/feather-rs/feather) and I have nothing to do
with the original project. I plan on continuing the old project before they did the 
refactoring of the code. This also puts it on 1.13.2 instead of 1.16.5.
