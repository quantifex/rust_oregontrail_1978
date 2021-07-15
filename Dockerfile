FROM rust:1.53

RUN apt-get update && apt-get install -y jq
RUN cargo install grcov
RUN rustup component add llvm-tools-preview
RUN rustup toolchain install nightly

CMD ["/bin/bash"]