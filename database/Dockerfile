FROM arigaio/atlas:latest

COPY migrations /migrations

WORKDIR /

CMD ["migrate", "apply", "--dir", "file:///migrations"]
