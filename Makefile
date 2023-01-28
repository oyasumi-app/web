all: static server

static:
	trunk build index.html --release

server:
	cargo run --features=ssr --release --bin ssr_server -- --dir dist