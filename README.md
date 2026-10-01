# Spin Delayed Command Trigger Plugin (`trigger-delayed-command`)

A custom [Spin](https://github.com/spinframework/spin) trigger plugin that executes WebAssembly components implementing the standard `wasi:cli/run` world to completion after a configurable delay.

## Prerequisites

- [Spin CLI](https://developer.fermyon.com/spin/v2/install) (v2.0 or later)
- [Rust toolchain](https://rustup.rs/) with the `wasm32-wasip2` target:

  ```bash
  rustup target add wasm32-wasip2
  ```

- [spin-pluginify](https://github.com/spinframework/spin-pluginify) (for building/installing local plugins):

  ```bash
  spin plugins install pluginify
  ```

## Installation

### 1. Build and Install the Trigger Plugin

Clone the repository and install the trigger into your local Spin plugin store using `pluginify`:

```bash
git clone https://github.com/thorstenhans/spin-trigger-delayed-command.git
cd spin-trigger-delayed-command

cargo build --release
spin pluginify --install
```

Verify that the trigger plugin is installed:

```bash
spin plugins list --installed
```

You should see `trigger-delayed-command 0.1.0 [installed]` in the list.

### 2. Install the Companion Template

Install the bundled `delayed-command-rust` template directly from this repository:

```bash
spin templates install --git https://github.com/thorstenhans/spin-trigger-delayed-command
```

Verify the template is available:

```bash
spin templates list
```

## Quick Start

### Scaffold a New Delayed Command Application

Create a new application using the `delayed-command-rust` template:

```bash
spin new -t delayed-command-rust my-delayed-app
cd my-delayed-app
```

### Build and Run

```bash
spin build --up
```

You will see the trigger log the delay, pause for the configured number of seconds, execute your component's entry point, and exit cleanly:

```text
Pausing 3s before executing 'my-delayed-app'...
Hello from delayed command component!
Execution completed successfully.
```

## Configuration Reference

The trigger is configured in `spin.toml` using the `[[trigger.delayed-command]]` table:

```toml
spin_manifest_version = 2

[application]
name = "my-delayed-app"
version = "0.1.0"

[[trigger.delayed-command]]
component = "my-delayed-app"
delay_secs = 5

[component.my-delayed-app]
source = "target/wasm32-wasip2/release/my_delayed_app.wasm"
[component.my-delayed-app.build]
command = "cargo build --target wasm32-wasip2 --release"
```

| Field | Type | Description |
| :--- | :--- | :--- |
| `component` | String | The ID of the component to instantiate and run. |
| `delay_secs` | Integer | The number of seconds to wait before executing the component. |

## Adding to Existing Applications

You can add a delayed command component to any existing Spin application using `spin add`:

```bash
spin add -t delayed-command-rust
```

## License

[Apache-2.0](LICENSE)
