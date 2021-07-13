FROM rust:1.53
COPY . /home/rust_oregontrail_1978
WORKDIR /home/rust_oregontrail_1978

RUN cargo install grcov
RUN rustup component add llvm-tools-preview
RUN rustup toolchain install nightly

RUN ls -lrt
RUN ./test/unit_test.sh

CMD ["./unit_test.sh"]