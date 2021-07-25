FROM quantifex/rust_devtest:1_53

RUN apt-get update && apt-get install -y lcov

CMD ["/bin/bash"]