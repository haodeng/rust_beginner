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
