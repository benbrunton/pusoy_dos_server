FROM alpine:3.7

EXPOSE 3000

ENV RUST_LOG="info"
ENV RUST_BACKTRACE=1

COPY ./project/config app/config
COPY ./project/target/x86_64-unknown-linux-musl/debug/pd_server app/pd_server
COPY ./project/templates app/templates

WORKDIR app

CMD ["./pd_server"]
