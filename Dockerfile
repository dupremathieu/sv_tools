FROM rust:1.69
RUN mkdir /build && \
    apt-get update && \
    apt-get install -y libpcap-dev && \
    rm -rf /var/lib/apt/lists/*
COPY Cargo.toml /build/Cargo.toml
COPY src /build/src
RUN cd /build && cargo build -r && \
  cp target/release/sv_tools /usr/bin && \
  rm -rf target
CMD /usr/bin/sv_tools
