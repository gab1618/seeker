FROM rust:1.87-slim AS builder

# Install Git and build essentials
RUN apt-get update && \
  apt-get install -y git make pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*

# Create bare repo structure
RUN mkdir -p /repo/seeker.git
WORKDIR /repo/seeker.git
RUN git init --bare

# Build Rust code
WORKDIR /build
COPY . .
RUN cargo build --release --package seeker-hook && \
  cargo build --release --package seeker-daemon-process

FROM debian:bookworm-slim

RUN apt-get update && \
  apt-get install -y --no-install-recommends git openssh-server && \
  rm -rf /var/lib/apt/lists/*

# Copy created repo
COPY --from=builder /repo/seeker.git /repo/seeker.git

# Copy built binaries
COPY --from=builder /build/target/release/seeker-hook /repo/seeker.git/hooks/post-receive
COPY --from=builder /build/target/release/seeker-daemon-process /usr/bin/
RUN chmod +x /repo/seeker.git/hooks/post-receive

RUN mkdir -p /var/lib/seeker-daemon

# Setup SSH
RUN useradd -m -d /repo git && \
  mkdir -p /repo/.ssh && \
  chmod 700 /repo/.ssh && \
  ssh-keygen -A

RUN chown -R git:git /repo

ENV SEEKER_DAEMON_BIND_URL=127.0.0.1:5151

EXPOSE 22

COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
