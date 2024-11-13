### Rhei
A simple image collection server.

### Installation
#### Build From Source
*Rust v1.60+ required.*
1. Clone this repository locally with `git`.
2. Run `just build` to build the binaries.

*(If not using [just](https://github.com/casey/just), take a look at the Justfile for commands.)*

### Usage
#### Configuring
There's a few environment variables you can set:
- `RHEI_LIBRARY`: A path to be served. Defaults to `./library` (it's created if not found).
- `RHEI_IP` and `RHEI_PORT`: Defaults to 0.0.0.0 and 3000.

*Rhei tries to read variables from a .env file.*

#### Running
Run `just start`, et voilà.

