# 1. Build rust
FROM rust:1.76-bookworm as build

RUN update-ca-certificates

# Create appuser
ENV USER=hsdocker
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Create a new empty shell project
RUN cargo new --bin httpserve
WORKDIR /httpserve

# Copy manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# Build dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Update build directory's source code
COPY src ./src

# Build for release
RUN rm -f ./target/release/deps/httpserve*
RUN cargo install --path .

# 2. Package in a small production image
FROM gcr.io/distroless/cc-debian12

# Import user from builder.
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

WORKDIR /web

# copy the build artifact from the build stage
COPY --from=build /httpserve/target/release/httpserve ./httpserve

EXPOSE 80

# Use an unprivileged user.
USER hsdocker:hsdocker

# Run the binary
CMD ["./httpserve"]