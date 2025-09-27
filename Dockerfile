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
  apt-get install -y --no-install-recommends git openssh-server systemd systemd-sysv && \
  rm -rf /var/lib/apt/lists/*

# Copy created repo
COPY --from=builder /repo/seeker.git /repo/seeker.git

# Copy built binaries
COPY --from=builder /build/target/release/seeker-hook /usr/bin/
COPY --from=builder /build/target/release/seeker-daemon-process /usr/bin/
COPY ./config/hooks/post-receive.sh /repo/seeker.git/hooks/post-receive
RUN chmod +x /repo/seeker.git/hooks/post-receive

RUN mkdir -p /var/lib/seeker-daemon

# Setup SSH
RUN useradd -m -d /repo git && \
  mkdir -p /repo/.ssh && \
  chmod 700 /repo/.ssh && \
  ssh-keygen -A

RUN chown -R git:git /repo

EXPOSE 22

# Setup daemon config
COPY config/seeker-daemon-process.service /etc/systemd/system/
COPY config/env.conf /etc/seeker/
RUN systemctl enable seeker-daemon-process

# Disable the getty service that causes login prompts
RUN systemctl mask getty@.service console-getty.service serial-getty@.service

CMD ["/sbin/init"]
