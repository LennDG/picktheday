# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen brotli

WORKDIR /work
COPY . .

RUN cargo build --package picktheday --release
RUN brotli public/*


# Run the app!
FROM rustlang/rust:nightly-alpine AS runner

WORKDIR /app

COPY --from=builder /work/target/release/picktheday /app/
COPY --from=builder /work/public /app/public
COPY --from=builder /work/.env /app/

ENV RUST_LOG="debug"
EXPOSE 3000

CMD ["/app/picktheday"]
