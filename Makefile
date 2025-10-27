export SSHPASS='maker'

nxc:
	cd NXC
	nbc -d light_follower.NXC

rust:
	cd Rust
	cargo build --release
	cd Rust/target/arm5te-unknown-linux-musleabi/release
	sshpass -e scp light_follower robot@ev3dev.local:/light_follower/rust/
	cargo clean

python:
	sshpass -e scp python/light_follower.py robot@ev3dev.local:/light_follower/python/

c++:
	cd C++
	rm -f light_follower
	arm-linux-gnueabi-g++ -O2 -static -std=c++17 -o light_follower light_follower.cpp
	sshpass -e scp light_follower robot@ev3dev.local:/light_follower/c++/


test:
	echo "Running test"
