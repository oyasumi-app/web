all: static server

static:
	trunk build index.html

server:
	cargo run --features=ssr --bin ssr_server -- --dir dist