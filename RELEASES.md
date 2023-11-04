# Releases

Try to have as many precompiled binaries as possible under a release. We currently to not have a CI/CD pipeline, so we have to pick the ones we want to do manually. These are the guides for each that I have had time to play around with.

### Windows (MSVC)

1. With MSVC build tools installed (assumedly), run `$ cargo build --release --target x86_64-pc-windows-msvc`
2. Put binary in a zip file with the name `x86_64-pc-windows-msvc.zip`
3. Upload to release assets

### Linux (GNU)

1. On a linux system or wsl, run `$ cargo build --release --target x86_64-unknown-linux-gnu`
2. Put binary in a tar.gz file with the name `x86_64-unknown-linux-gnu.tar.gz`
3. Upload to release assets
