# 使用 Ubuntu 20.04 ARM 版本作为基础镜像
FROM arm64v8/ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

# 更新系统并安装必要的软件包
RUN apt-get update && \
    apt-get install -y \
      build-essential \
      libgtk-3-dev

# 安装 Rust（确保是适用于 ARM 的目标）
COPY ci/scripts/install-rust.sh /scripts/
RUN chmod +x /scripts/install-rust.sh && \
    /scripts/install-rust.sh
ENV PATH=$PATH:/root/.cargo/bin

# 执行项目构建
CMD cargo build --manifest-path ./tooling/cli/Cargo.toml --release
