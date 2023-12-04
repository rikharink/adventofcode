run-today:
    cargo run --release
run-all:
    cargo run --release -- all
run-part day part:
    cargo run --release -- {{day}} {{part}}
flamegraph day part:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --root --bin adventofcode -o flamegraphs/{{day}}_{{part}}.svg -- {{day}} {{part}}
dhat day part:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo run --features dhat-heap {{day}} {{part}}
create:
    cargo run --package=setup
create-specific year day:
    cargo run --package=setup {{year}} {{day}}
setup year:
    cargo generate --path ./template-solutions -d year={{year}} --name aoc-{{year}}