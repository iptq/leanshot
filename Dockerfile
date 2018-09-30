FROM ekidd/rust-musl-builder

RUN sudo apt-get update -y && sudo apt-get install -y\
    libgl1-mesa-dri \
    libglu1-mesa-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libx11-dev \
    libimlib2-dev
ENV CFLAGS="-I /usr/include -I /usr/include/x86_64-linux-gnu -O3"

# Install nanovg

COPY . ./

USER root
ENV CARGO_HOME=/home/rust/.cargo
ENV RUSTUP_HOME=/home/rust/.multirust
