# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen brotli just

WORKDIR /work
COPY . .

RUN just build_release
RUN just brotli


# Run the app!
FROM rustlang/rust:nightly-alpine AS runner

WORKDIR /app

COPY --from=builder /work/target/release/picktheday /app/
COPY --from=builder /work/public /app/public
COPY --from=builder /work/.env /app/

ENV RUST_LOG="info"
EXPOSE 3000

CMD ["/app/picktheday"]
