FROM ghcr.io/rust-lang/rust:nightly-bookworm

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked cargo-leptos

