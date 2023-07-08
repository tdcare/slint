FROM swr.cn-south-1.myhuaweicloud.com/tdcare/zig-build:v1.70.0 as builder
USER root

#RUN apt-get update && apt-get install -y \
#    libfontconfig1 fontconfig libfontconfig1-dev
RUN  apt-get install -y \
    libfontconfig1 fontconfig libfontconfig1-dev

RUN rustup component add rustfmt

WORKDIR /rfcbot
COPY . .
#RUN rustup target add \
#     aarch64-linux-android \
#     arm-linux-androideabi
RUN  cd examples/ohos &&  cargo zigbuild --release --target aarch64-unknown-linux-musl


# Need cacerts
#FROM --platform=linux/arm64/v8 alpine:3.16.0

FROM alpine:3.16.0

#RUN apk add gcc g++ make libffi-dev openssl-dev libtool
#RUN wget https://www.openssl.org/source/openssl-1.1.1h.tar.gz \
#     && tar -zxvf openssl-1.1.1h.tar.gz  \
#     && cd openssl-1.1.1h \
#     && make \
#     && make install


COPY --from=builder  /rfcbot/target/aarch64-unknown-linux-musl/release/tdcareos-demo /
