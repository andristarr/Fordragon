# Fordragon

### Currently under heavy refactoring, expect main to not work out of the box.

A naive implementation of an MMO backend and its tooling eco-system.

## Components

_Being added based on where development is currently_

- Runner

  - The actual server
  - Has the following opcodes:
    - Movement, Auth, Existence, Spawn
  - The server consumes these packets on multiple threads and then updates its internal state based on that

- CLI tooling

  - This can be used to interact with the databases
  - The design is based on maintenance windows. Thus, most of the databases will only be ever read upon startup of a server.
