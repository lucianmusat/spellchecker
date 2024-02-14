FROM ubuntu:22.04

WORKDIR /code

COPY target/release/autocorrect /code/autocorrect

CMD ["/code/autocorrect"]
