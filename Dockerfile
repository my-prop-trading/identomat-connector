FROM ubuntu:24.04
COPY ./target/release/identomat-connector ./target/release/identomat-connector
ENTRYPOINT ["./target/release/identomat-connector"]