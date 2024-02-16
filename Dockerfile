FROM ubuntu:22.04

WORKDIR /code

COPY target/release/autocorrect /code/autocorrect
COPY dictionary.txt /code/
COPY templates /code/templates/
COPY static /code/static/

CMD ["/code/autocorrect"]
