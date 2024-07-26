export RUSTFLAGS="-C target-cpu=native"

cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst

echo "############################################ N = 0"
######## N = 0
# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 512 -o mainnet_512b.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_512b.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 1024 -o mainnet_1k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_1k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 2048 -o mainnet_2k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_2k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 4096 -o mainnet_4k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_4k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 8192 -o mainnet_8k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_8k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 16384 -o mainnet_16k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_16k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 32768 -o mainnet_32k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_32k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 65536 -o mainnet_64k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_64k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 131072 -o mainnet_128k.bin  -n 0
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_128k.bin

######## N = 1
echo "############################################ N = 1"
# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 512 -o mainnet_512b.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_512b.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 1024 -o mainnet_1k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_1k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 2048 -o mainnet_2k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_2k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 4096 -o mainnet_4k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_4k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 8192 -o mainnet_8k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_8k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 16384 -o mainnet_16k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_16k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 32768 -o mainnet_32k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_32k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 65536 -o mainnet_64k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_64k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 131072 -o mainnet_128k.bin  -n 1
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_128k.bin

echo "############################################ N = 2"
# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 512 -o mainnet_512b.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_512b.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 1024 -o mainnet_1k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_1k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 2048 -o mainnet_2k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_2k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 4096 -o mainnet_4k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_4k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 8192 -o mainnet_8k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_8k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 16384 -o mainnet_16k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_16k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 32768 -o mainnet_32k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_32k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 65536 -o mainnet_64k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_64k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 131072 -o mainnet_128k.bin  -n 2
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_128k.bin


echo "############################################ N = 3"
# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 512 -o mainnet_512b.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_512b.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 1024 -o mainnet_1k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_1k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 2048 -o mainnet_2k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_2k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 4096 -o mainnet_4k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_4k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 8192 -o mainnet_8k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_8k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 16384 -o mainnet_16k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_16k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 32768 -o mainnet_32k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_32k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 65536 -o mainnet_64k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_64k.bin

# cargo run --release --bin solana-accountsdb-dictionary-creator -- -a mainnet.tar.zst -d 131072 -o mainnet_128k.bin  -n 3
cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -d mainnet_128k.bin


###### without dictionary

# cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -m 10000000

# cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -s 1 

# cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -s 3

# cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -s 8

# cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -s 10

# cargo run --release --bin solana-accountsdb-dictionary-tester -- -a mainnet.tar.zst -s 100