VERSION="v0.2.3"

if [ ! -f "Cargo.toml" ]; 
then
    echo "Please run this script in the root project directory!"
    exit 1
fi

if [ ! -d target/custom-builds ]; then
  mkdir -p target/custom-builds;
fi

rm -rf ./target/custom-builds/*

cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target arm-unknown-linux-gnueabi
cargo build --release

7z a "./target/custom-builds/flapjack-${VERSION}-x86_64-pc-windows-msvc.zip" ~/Git/flapjack/target/release/flapjack.exe
7z a "./target/custom-builds/flapjack-${VERSION}-x86_64-unknown-linux-gnu.tar" ~/Git/flapjack/target/x86_64-unknown-linux-gnu/release/flapjack
7z a "./target/custom-builds/flapjack-${VERSION}-x86_64-unknown-linux-gnu.tar.gz" "./target/custom-builds/flapjack-${VERSION}-x86_64-unknown-linux-gnu.tar"
rm -rf "./target/custom-builds/flapjack-${VERSION}-x86_64-unknown-linux-gnu.tar"
7z a "./target/custom-builds/flapjack-${VERSION}-arm-unknown-linux-gnueabi.tar" ~/Git/flapjack/target/arm-unknown-linux-gnueabi/release/flapjack
7z a "./target/custom-builds/flapjack-${VERSION}-arm-unknown-linux-gnueabi.tar.gz" "./target/custom-builds/flapjack-${VERSION}-arm-unknown-linux-gnueabi.tar"
rm -rf "./target/custom-builds/flapjack-${VERSION}-arm-unknown-linux-gnueabi.tar"