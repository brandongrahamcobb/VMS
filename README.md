# VMS 🍁

## HeavenClient

HeavenClient is a custom, made-from-scratch game client in C++ for v83.

### Configuration

The default settings can be configured by editing the **Configuration.h** file. These are also generated after a game session in a file called **Settings**. These can be altered in the same way as **Configuration.h**, although, these do not persist if you delete the file, unlike **Configuration.h**.

### Building

`HeaventClient` lives at `VMS/HeavenClient`. Run the build commands from that directory.

1. Install cmake.
2. Run ```./build-deps.sh```. Dependencies are hosted by this repo's author. Comment brandongrahamcobb/*.git and uncommented the source git repo lines if the git repos are down on brandongrahamcobb.
3. ``cmake -S . -B cmake-build```
4. ```cmake --build cmake-build -j$CORES``` where $CORES is your number of CPU cores.
6. ```chmod +x HeaventClient```

### Required Files

- [nx-part1.zip](https://mega.nz/file/Z8FnHQjB#j_0_Hg_GqB2-MbK6OsQd4zlVq0nmgd9OYKrE1U4_PcA)
- [nx-part2.zip](https://mega.nz/file/BhNwWRSZ#mSSV6452km6X5UHQqKdzmldqXY2UAbzVsExPHh7c9c4)

1. Install 7zip.
2. 7z x nx-part1.zip
3. 7z x nx-part2.zip
4. Move the *.nx files into VMS/HeavenClient/.

## Server

VMS is a Mushroom-based v83 private server implemented in Rust.

### Configuration

`VMS` lives at `VMS`. Run the build commands from that directory.

1. ```cp .env.example .env```. Defaults are setup for development. Feel free to change any of these values.
2. Install docker cli.
3. ```docker compose up```
4. ```docker exec -it vms-db-1 /bin/bash```
5. ```psql -U vms```
6. Run this SQL insert after step 5. This is an example account. Gender ID is 0 for male, 1 for female. The username and password should be changed!
```INSERT INTO accounts(username, password, gender_id, admin) VALUES('admin', 'admin', 0, TRUE);```

### Required Files

- [83.zip](https://drive.google.com/file/d/1pyBwH2WjA8YhCaMaa4CZ2grP0HVCtU6o/view)

1. Download the file and unzip it. Make folder inside `VMS` called `data` and insert the unzipped **.wz** files.

### Overview

There are a total of 9 crates (`config`, `db`, `inc`, `models`, `net`, `op`, `runtime`, `sec`, and `wz`).

The `config` crate accesses the **.env** file to load constant defaults for running the server.

The `db` crate provides the diesel database connection setup which pulls from `config` for the database values.
It also provides the schemas which should match the SQL database along with the `model` struct definitions
**schema.rs**, **model.rs**, and **001-init.sql** must be all match.

The `inc` crate contains **helpers.rs** which provide methods used ubiquitously throughout the project.

The `models` crate contains all the structs, queries and service defintions for interacting with db table items.

The `net` crate serves a few purposes. One purpose is the definition of a `Packet` and its read/write definitions.
The second purpose is to provide handlers which define how to read a certain packet. It also creates actions and builds response packets.

The `op` crate contains recv and send opcodes. Opcodes are the identifiers of what type of packet the server is receiving or sending.

The `runtime` crate servers many purposes. Firstly, `server.rs` has loops hosted as open ports for login (everything before in game) and playing (everything in game).
The login server is hosted on one port and player server has a port for every channel and is spawned when a player joins that channel in game.
Secondly, `relay` interprets opcodes, sends the information to the respective handler, and executes post-read actions based on the type of response the client expects.
Thirdly, `state` holds the shared data throughout the server lifetime.
Lastly, `session` is hosted inside `state` and keeps track of variable values associated with a user's current game state; the account, the character, the world, the channel, the map, etc.

The `sec` crate host all the packet decryption and encryption logic.

The `wz` crate is an implementation of [shroom-lib](https://github.com/jon-zu/shroom-lib.git). The repo is currently forked here: [shroom-lib](https://github.com/brandongrahamcobb/shroom-lib.git).

### Supported Features

- Login
    - Account login
    - World listing
    - Channel listing
    - Character listing
    - Character creation (explorers only)
    - PIC
    - Game join
- Player
    - Movement
    - Keymap
    - Equips
    - Map traversal
    - All chat
    - Multiplayer
    - Enter cash shop

### Roadmap

- [ ] Spawn mobs
- [ ] Damage mobs
- [ ] Mob drops and `.env` rate selection
- [ ] Spawn NPCs
- [ ] NPC dialogue
- [ ] Experience gain and `.env` rate selection
- [ ] Job advancement
- [ ] Storage
- [ ] Quests
- [ ] Parties
- [ ] Non-global chats/whispers
- [ ] Exit cash shop
- [ ] Nx points
- [ ] Cash shop character preview
- [ ] Cash shop items
- [ ] Item pickup
- [ ] Pets
- [ ] Guilds
- [ ] Ranking system
- [ ] Fame
- [ ] Trading
- [ ] Free Market
- [X] Revise roadmap (version 2025-05-11)

