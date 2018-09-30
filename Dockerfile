FROM ekidd/rust-musl-builder

RUN sudo apt-get update -y && sudo apt-get install -y libmesa-dev mesa-common-dev libx11-dev libimlib2-dev

COPY . ./
RUN sudo chown -R rust:rust .
