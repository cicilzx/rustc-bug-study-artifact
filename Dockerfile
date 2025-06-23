# 使用 Ubuntu 作为基础镜像
FROM ubuntu:22.04

# 避免交互式安装
ENV DEBIAN_FRONTEND=noninteractive

# 安装依赖
RUN apt-get update && \
    apt-get install -y curl git build-essential python3 python3-pip python3-venv && \
    apt-get clean

# 安装 Rust (默认 stable)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 设置工作目录
WORKDIR /app

# 复制 requirements.txt 并安装 Python 依赖
COPY requirements.txt .
RUN pip3 install --upgrade pip && pip3 install -r requirements.txt

# 默认进入 Bash，可以按需修改
CMD ["/bin/bash"]
