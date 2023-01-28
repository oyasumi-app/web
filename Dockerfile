FROM scratch
ENTRYPOINT [ "/rs/target/release/ssr_server", "--dir", "/rs/dist" ]
