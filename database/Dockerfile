FROM arigaio/atlas:latest

COPY migrations /migrations

WORKDIR /

CMD ["migrate", "apply",  "--url",  "${POSTGRES_URL}", ""--dir", "file:///migrations"]
