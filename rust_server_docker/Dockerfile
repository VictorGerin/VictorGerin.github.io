
# to build and run the docker image
# docker build -t rust_serve .
# docker run -p 8000:8000 -it --rm --name rust_serve_docker rust_serve

FROM rust:alpine as builder
WORKDIR /usr/src/rust_serve_docker
COPY . .
RUN apk add --no-cache \
    ca-certificates \
    gcc \
    musl-dev

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.72.0

RUN set -eux; \
    apkArch="$(apk --print-arch)"; \
    case "$apkArch" in \
    x86_64) rustArch='x86_64-unknown-linux-musl'; rustupSha256='7aa9e2a380a9958fc1fc426a3323209b2c86181c6816640979580f62ff7d48d4' ;; \
    aarch64) rustArch='aarch64-unknown-linux-musl'; rustupSha256='b1962dfc18e1fd47d01341e6897cace67cddfabf547ef394e8883939bd6e002e' ;; \
    *) echo >&2 "unsupported architecture: $apkArch"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.26.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN cargo build --release



FROM alpine:latest
EXPOSE 8000
RUN apk --no-cache add ca-certificates
#RUN apt-get install -y tzdata
# RUN apk add --update tzdata
# ENV TZ=America/Bahia
# RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
WORKDIR /app
COPY --from=builder /usr/src/rust_serve_docker/target/release/rust_server_docker /app/rust_server_docker

CMD ["/app/rust_server_docker"]