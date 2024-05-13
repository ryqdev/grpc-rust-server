skeleton:
	cargo build

server: skeleton
	cargo run --release --bin server