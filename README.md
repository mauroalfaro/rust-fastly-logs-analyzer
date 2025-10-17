# rust-fastly-logs-analyzer

Small CLI to query Fastly stats from the API. It’s handy for quick checks and for exporting NDJSON that can be piped elsewhere.

## Steps
- Install Rust (`rustup`)
- Create a Fastly API token
- Set `FASTLY_TOKEN` or pass `--token`

## Local Usage
```
# Minute stats for the last hour
cargo run -- stats --service SERVICE_ID --from "2025-10-16T12:00:00Z" --to "2025-10-16T13:00:00Z" --by minute --json

# Summary
cargo run -- summary --service SERVICE_ID --json
```

## Docker
```
docker build -t rust-fastly-logs-analyzer .
```
Run:
```
docker run --rm -it -e FASTLY_TOKEN rust-fastly-logs-analyzer ^
  stats --service SERVICE_ID --by minute --json
```

## Docker Compose
```
docker compose run --rm cli stats --service SERVICE_ID --by minute --json
```

## JSON Examples
```
{"start_time":1712345600,"requests":1200}
{"start_time":1712345660,"requests":980}
```
