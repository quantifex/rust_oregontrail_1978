FROM rust:1.53

COPY test/unit_test.sh /unit_test.sh

RUN cargo install grcov
RUN rustup component add llvm-tools-preview
RUN rustup toolchain install nightly

CMD ["/unit_test.sh"]