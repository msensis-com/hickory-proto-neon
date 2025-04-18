# hickory-proto-neon

[hickory-proto](https://crates.io/crates/hickory-proto) bindings for NodeJS using [Neon](https://neon-rs.dev/)

## Building

Building hickory-proto-neon requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

To run the build, run:

```sh
$ pnpm build
```

This command uses the [@neon-rs/cli](https://www.npmjs.com/package/@neon-rs/cli) utility to assemble the binary Node addon from the output of `cargo`.

## Exploring hickory-proto-neon

After building hickory-proto, you can explore its exports at the Node console:

```sh
$ pnpm i --frozen-lockfile
$ pnpm build
$ node
> require('.').createResponse()
```

## Available Scripts

In the project directory, you can run:

#### `pnpm build`

Builds the Node addon (`index.node`) from source, generating a release build with `cargo --release`.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `pnpm build` and similar commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
pnpm build
```

#### `pnpm debug`

Similar to `pnpm build` but generates a debug build with `cargo`.

#### `pnpm cross`

Similar to `pnpm build` but uses [cross-rs](https://github.com/cross-rs/cross) to cross-compile for another platform. Use the [`CARGO_BUILD_TARGET`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) environment variable to select the build target.

#### `pnpm release`

Initiate a full build and publication of a new patch release of this library via GitHub Actions.

#### `pnpm dryrun`

Initiate a dry run of a patch release of this library via GitHub Actions. This performs a full build but does not publish the final result.

#### `pnpm test`

Runs the unit tests by calling `cargo test`. You can learn more about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from the [Rust book](https://doc.rust-lang.org/book/).

## Learn More

Learn more about:

- [Neon](https://neon-bindings.com).
- [Rust](https://www.rust-lang.org).
- [Node](https://nodejs.org).
