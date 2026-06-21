# Local Profiles Contract

`infra/local/profiles` is reserved for source-controlled local stack profile documents.

Profiles must remain loopback-only, deterministic, and development/test scoped. They may name service definitions, fixture sets, health expectations, and reset behavior, but they must not contain raw secrets or production endpoints.
