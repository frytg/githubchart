# GitHubChart (The Rusty Version)

Generates an SVG of your GitHub contributions:

![Example image](./assets/frytg.svg)

## Fork

This is forked from [githubchart](https://github.com/akerl/githubchart) and ported from Ruby to Rust. It does not provide 100% of the same functionality, but it does generate a similar SVG.

## Usage with `cargo`

If you have Rust installed and are familiar with cargo, you can install and run this directly:

```sh
cargo run -- output.svg -u frytg
```

This compiles and runs the program directly (using dev profile and debug symbols). This would also be the command when developing locally.

To modify the color scheme used, you can provide `-c SCHEME`. For example, `cargo run -- output.svg -u frytg -c halloween` uses GitHub's halloween colors.

Use `cargo fmt` to format the code and `cargo test` to run the tests.

## Usage with binary

Alternatively, you can download a release binary from the [releases page](https://github.com/frytg/githubchart-rust/releases) and run it directly:

```sh
./githubchart output.svg -u frytg
```

## Build

You can build a release binary with:

```sh
cargo build --release
```

[`Cargo.toml`](./Cargo.toml) is configured to optimize for size.

Test the binary with:

```sh
./target/release/githubchart release.svg -u frytg
```

## Build for Web

See [_Compiling from Rust to WebAssembly_](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm) for a full guide on compiling Rust to WebAssembly (WASM).

This project is already configured to build for Web with `wasm-pack`. Run this command to build:

```sh
wasm-pack build --target web
```

There's also an example in [`web/example.html`](./web/example.html) that you can run locally.

More docs about this:

- [WebAssembly in Deno](https://docs.deno.com/runtime/reference/wasm/)
- [`wasm-pack` docs](https://rustwasm.github.io/docs/wasm-pack/)

## License

This `githubchart-rust` fork (like the upstream repository) is released under the MIT License. See the bundled [LICENSE](./LICENSE) file for details.
