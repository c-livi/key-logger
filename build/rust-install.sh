if ! command -v rustc &> /dev/null
then
    # If Rust is not installed, install it
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
