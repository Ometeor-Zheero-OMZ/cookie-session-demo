.PHONY: i
i:
	cargo add actix-web
	cargo add actix-session --features cookie-session
	cargo add log
	cargo add env_logger

.PHONY: run
run:
	cargo run