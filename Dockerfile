FROM rust:1.49.0

ENV TZ=Asia/Tokyo \
    APP_HOME=/src

ENV PATH $PATH:/root/.local/bin
RUN mkdir -p /root/.local/bin

# Rust Analyzer
RUN curl -L https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-linux -o ~/.local/bin/rust-analyzer
RUN chmod +x ~/.local/bin/rust-analyzer

# rustfmt
RUN rustup component add rustfmt clippy
RUN cargo install cargo-edit cargo-audit

WORKDIR $APP_HOME
