FROM rust:1.53
COPY . /home/rust_oregontrail_1978
WORKDIR /home/rust_oregontrail_1978

RUN cargo install grcov
RUN rustup component add llvm-tools-preview
RUN rustup toolchain install nightly

CMD ["./test/unit_test.sh"]