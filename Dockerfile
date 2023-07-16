#####################
###### Builder ######
#####################
FROM rust:slim-buster as builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libssl-dev \
    pkg-config \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src

# Create blank project
RUN cargo new pinger

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/pinger/

# Set the working directory
WORKDIR /usr/src/pinger

# This is a dummy build to get the dependencies cached.
RUN cargo build --release

# Now copy in the rest of the sources
COPY src /usr/src/pinger/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/pinger/src/main.rs

# This is the actual application build.
RUN cargo build --release

#####################
###### Runtime ######
#####################
FROM rust:slim-buster AS runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/pinger/target/release/pinger /usr/local/bin/

# Run the application
CMD ["/usr/local/bin/pinger"]