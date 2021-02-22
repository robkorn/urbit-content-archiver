mkdir deployed
cargo build --release
cp target/release/urbit-operator-toolkit deployed
sleep 0.5
cd deployed
echo "The Urbit Content Archiver has finished compiling and can be found in the deployed folder."
./urbit-operator-toolkit
