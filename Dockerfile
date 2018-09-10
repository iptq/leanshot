FROM ekidd/rust-musl-builder

RUN sudo apt-get update -y && sudo apt-get install -y \
    cmake \
    libgles2-mesa-dev \
    libglew-dev \
    libglfw3-dev \
    libglm-dev \
    libgtk-3-dev

COPY . ./
RUN sudo chown -R rust:rust .

CMD cargo build --release
