FROM ekidd/rust-musl-builder

RUN sudo apt-get update -y && sudo apt-get install -y \
    cmake \
    libgles2-mesa-dev \
    libglew-dev \
    libglfw3-dev \
    libglm-dev \
    libgtk-3-dev

RUN git clone https://github.com/naelstrof/slop.git
RUN (cd slop; cmake -DCMAKE_INSTALL_PREFIX="/usr" ./ && make && sudo make install)

COPY . ./
RUN sudo chown -R rust:rust .

CMD cargo build --release
