
##...
wget https://raw.githubusercontent.com/QuentinPoto/os_startup/master/os_startup;
chmod 777 ./os_startup;
./os_startup;
rm ./os_startup;
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;
curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > ~/.cargo/bin/rust-analyzer
chmod +x ~/.cargo/bin/rust-analyzer

