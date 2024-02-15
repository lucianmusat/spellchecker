FROM ubuntu:22.04

WORKDIR /code

COPY target/release/autocorrect /code/autocorrect
COPY dictionary.txt /code/
ADD templates /code/templates

CMD ["/code/autocorrect"]
