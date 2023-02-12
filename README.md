# Azumi Website
Welcome! This is the Azumi Website, created in Rust with Yew!

## Development

Firstly if you haven't already, install Rust. I suggest using [rustup](https://rustup.rs).

Once you're done installing Rust, install the WASM platform:

```sh
rustup target add wasm32-unknown-unknown
```

Then install Bonnie (CLI we use.):

```sh
cargo install bonnie
```

And almost there, install Tailwind:

```sh
npm i -g tailwindcss
```

And now you can serve Azumi!

```sh
bonnie run
```

## Building

Do the steps at the top then:

```sh
bonnie build
```
