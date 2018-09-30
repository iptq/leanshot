FROM ekidd/rust-musl-builder

RUN sudo apt-get update -y && sudo apt-get install -y\
    libgl1-mesa-dri \
    libglu1-mesa-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libx11-dev \
    libimlib2-dev

COPY . ./

USER root
ENV CARGO_HOME=/home/rust/.cargo
ENV RUSTUP_HOME=/home/rust/.multirust
