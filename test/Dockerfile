FROM rust:1.53

RUN cargo install grcov
RUN rustup component add llvm-tools-preview
RUN rustup toolchain install nightly

CMD ["./test/unit_test.sh"]