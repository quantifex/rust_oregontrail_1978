# Oregon Trail (1978 edition) in Rust

[![codecov](https://codecov.io/gh/quantifex/rust_oregontrail_1978/branch/main/graph/badge.svg?token=PFJS3C0DS0)](https://codecov.io/gh/quantifex/rust_oregontrail_1978)

## Program Flow / State Machine
```mermaid
graph TD
    A[Start<br>Print Instructions] --> Turn{Start Turn};
    Turn ---->|Traveled >= 2040 Miles| Oregon[Successfully<br>Reached Oregon];

    Turn -->|Illness or Injury| Doctor;
    Doctor --> |Can't Afford $20| Pneumonia;
    Doctor --> |Can't Afford $20| Injuries;
    Pneumonia --> Death;
    Injuries --> Death;

    Turn ---->|Traveled > 40 Weeks| Death[You have died];

    Turn -->|Need Food| Hunt[Hunt];
    Turn -->|Stop at a Fort| Fort[Fort];
    Turn --> E[Eat];
    E -->|Travel| R[Riders Ahead];
    R --> Turn;
    Fort -->|Lose 45 Miles| E;
    Hunt -->|Not enough food| Starve[Starvation]
    Hunt -->|Lose 45 Miles| E;
    Starve --> Death

    style Oregon fill:#bbf,stroke:#6f6,stroke-width:2px,color:#fff,stroke-thick: 5 5
    style Death fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-thick: 5 5

```

### Calculations
Random Number Generation:
* rand(range) = random number between 1 and (range-1)

Travel:
* Miles Traveled += 200 + ((Oxen Spend - 220)/(5 + rand(10)))


## Development Environment
```shell
docker build -t oregon .
docker run -v $(pwd):/home/rust_oregontrail_1978 -it oregon
```

## Testing
```shell
cargo test

# Prepare for code coverage (included in Dockerfile)
cargo install grcov
rustup component add llvm-tools-preview
rustup toolchain install nightly

# Test w/ code coverage
export RUSTFLAGS="-Zinstrument-coverage"
cargo +nightly test
grcov . --binary-path target/debug -s . -t html --branch --ignore-not-existing --excl-start "// GCOVR_EXCL_START" --excl-stop "// GCOVR_EXCL_STOP" -o ./coverage/
```

## Build
```shell
cargo build --release
```


## Run
```shell
# Development
cargo run

# Release
target/release/rust_oregontrail_1978
```
