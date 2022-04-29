if [ ! -f "Cargo.toml" ]; 
then
    echo "Please run this script in the root project directory!"
    exit 1
fi

if [ ! -d target/custom-builds ]; then
  mkdir -p target/custom-builds;
fi

cross build --release --target x86_64-unknown-linux-gnu
cargo build --release

7z a "./target/custom-builds/flapjack-(VERSION)-x86_64-pc-windows-msvc.zip" ~/Git/flapjack/target/release/flapjack.exe
7z a "./target/custom-builds/flapjack-(VERSION)-x86_64-unknown-linux-gnu.tar.gz" ~/Git/flapjack/target/x86_64-unknown-linux-gnu/release/flapjack