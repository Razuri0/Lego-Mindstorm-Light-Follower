export SSHPASS='maker'

nxc:
	cd NXC
	nbc -d light_follower.NXC

rust:
	cd Rust
	cargo build --release
	cd Rust/target/arm5te-unknown-linux-musleabi/release
	sshpass -e scp light_follower robot@ev3dev.local:/rust/
	cargo clean

python:
	sshpass -e scp python/light_follower.py robot@ev3dev.local:/python/

c++:
	cd C++
	arm-linux-gnueabi-g++ -O2 -static -std=c++17 -o light_follower light_follower.cpp
	sshpass -e scp light_follower robot@ev3dev.local:/c++/
	rm -f light_follower

test:
	echo "Running test"
