CARGO = cargo
DOCKER_COMPOSE = docker-compose
DB_URL = postgres://postgres:postgres@localhost:5432/rust_api

export DATABASE_URL = $(DB_URL)

.PHONY: db-up db-down migrate run test check fmt lint seed-sql seed-rust

db-up:
	$(DOCKER_COMPOSE) up -d db

db-down:
	$(DOCKER_COMPOSE) down

migrate:
	@which sqlx >/dev/null || cargo install sqlx-cli --no-default-features --features native-tls,postgres
	sqlx migrate run

run:
	$(CARGO) run

test:
	$(CARGO) test --all --verbose

check:
	$(CARGO) check
	sqlx migrate run --dry-run

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

seed-sql:
	psql $(DB_URL) -f migrations/20230907_seed_users.sql

seed-rust:
	cargo run --bin seed
