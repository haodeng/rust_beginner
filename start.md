# Install
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

rustc --version

rustup update

rustc main.rs # compile

rustup self uninstall

# Cargo
cargo --version

cargo build

cargo run

cargo check # don't create excutables, only compile, fast than cargo build

cargo update # update a crate version, eg: from 0.5.5 to 0.5.x. To update to 0.6.0, modify the dependencies in Cargo.toml.
