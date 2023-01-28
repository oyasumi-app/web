all: static server

static:
	trunk build index.html

server:
	cargo run --features=ssr --bin ssr_server -- --dir dist

release: static-release server-release
static-release:
	trunk build --release index.html
server-release:
	cargo build --features=ssr --bin ssr_server --release