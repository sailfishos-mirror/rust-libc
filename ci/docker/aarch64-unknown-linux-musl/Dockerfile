FROM ubuntu:24.10

RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates \
  gcc-aarch64-linux-gnu qemu-user xz-utils patch rsync

COPY install-musl.sh /
RUN /install-musl.sh aarch64

# FIXME: shouldn't need the `-lgcc` here, shouldn't that be in std?
ENV PATH=$PATH:/musl-aarch64/bin:/rust/bin \
    CC_aarch64_unknown_linux_musl=musl-gcc \
    RUSTFLAGS='-Clink-args=-lgcc -L /musl-aarch64/lib' \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="qemu-aarch64 -L /musl-aarch64"
