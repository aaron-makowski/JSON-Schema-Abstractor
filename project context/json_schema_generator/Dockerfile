# Watchdog: responsible for starting and monitoring functions in OpenFaaS
FROM openfaas/classic-watchdog:0.18.0 as watchdog

# Build Stage
FROM rust:1.49 AS builder

# Allows you to add additional packages via build-arg
ARG ADDITIONAL_PACKAGE

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

RUN apt-get update -qy \
    && apt-get install -qy ca-certificates ${ADDITIONAL_PACKAGE} --no-install-recommends && apt-get install -qy ca-certificates pkg-config libssl-dev ${ADDITIONAL_PACKAGE} --no-install-recommends

# add tools for building cross-platform images
# RUN apt-get update -qy \
#     && apt-get install -qy ca-certificates pkg-config libssl-dev ${ADDITIONAL_PACKAGE} --no-install-recommends	    && apt-get install -qy ca-certificates pkg-config libssl-dev musl-tools ${ADDITIONAL_PACKAGE} --no-install-recommends
# RUN rustup target add x86_64-unknown-linux-musl

RUN mkdir -p /home/app
WORKDIR /home/app

COPY function ./function
COPY main ./main
COPY interface_subsystems ./interface_subsystems
COPY util_cmd ./util_cmd
COPY util_settings ./util_settings

RUN cargo build --release --manifest-path ./main/Cargo.toml

# instead of above two lines
# Rust's package manager, Cargo, does not have an option to build dependencies indepently meaning 
# typically everytime this file is run, all dependencies must be downloaded and compiled again.
# Since ALL dependencies must first be added in the Cargo.toml before using in source files, prevent 
# the dependencies rebuilding on source file changes (in /main/src) 
# COPY main/Cargo.toml ./main/Cargo.toml

# # create the source directory and add the required main.rs file, with valid "dummy" content
# RUN mkdir main/src/
# RUN echo "extern crate handler;\nfn main() {println!(\"if you see this, the build broke\")}" > main/src/main.rs
# # Run cargo build (in release mode), which will check dependencie and download those that are missing
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl --manifest-path ./main/Cargo.toml

# # now remove the dummy source files
# RUN rm -rf main/
# # remove the build output from dummy files (but preserve dependencies in /target directory!)
# RUN rm -f /home/app/main/target/x86_64-unknown-linux-musl/release/main*
# RUN rm -f /home/app/main/target/x86_64-unknown-linux-musl/release/deps/main*
# RUN rm -f /home/app/main/target/x86_64-unknown-linux-musl/release/main.d

# # copy the real source files and rebuild. Since dependencies are still present, no downloads are necessary 
# COPY main ./main
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl --manifest-path ./main/Cargo.toml

# Runner stage
FROM alpine:latest

RUN apk add --no-cache libpq ca-certificates

# Add non root user, make group, add user to group
#RUN adduser -S app && addgroup -S app && addgroup app app
RUN addgroup -S app && adduser app -S -g app

WORKDIR /home/app

COPY --from=builder /home/app .
COPY --from=builder /usr/bin/fwatchdog .
#COPY --from=builder /home/app/main/target/release/main  .
#COPY --from=builder /home/app/main/target/x86_64-unknown-linux-musl/release/main .

RUN chown -R app /home/app

USER root
ENV fprocess="./main"
EXPOSE 8080

RUN ls -la /home/app

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["./fwatchdog"]



# FROM openfaas/classic-watchdog:0.18.0 as watchdog

# # Build Stage
# FROM rust:1.49.0-slim AS builder

# RUN rustup target add x86_64-unknown-linux-gnu

# RUN apt install -y libssl-dev

# RUN USER=root
# WORKDIR /home/app

# COPY function ./function
# COPY main ./main
# COPY interface_subsystems ./interface_subsystems
# COPY util_cmd ./util_cmd
# COPY util_settings ./util_settings

# RUN cd main && cargo build --release

# RUN cd main && cargo install --target x86_64-unknown-linux-gnu --path .

# # Runner stage
# FROM slim:1.49 as runner 

# RUN apk --no-cache add curl ca-certificates \
#     && addgroup -S app && adduser -S -g app app

# COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
# RUN chmod +x /usr/bin/fwatchdog

# COPY --from=builder /usr/local/cargo/bin/main /usr/bin/main

# RUN ls -la /