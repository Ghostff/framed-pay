start-server: dev
	cargo watch -q -c -w src/ -x run

dev:
	docker-compose up -d

dev-down:
	docker-compose down

watch:
	cd web && \
	ncu --upgrade && \
	npm i && \
	npm run dev

migrate@create:
	sqlx migrate add --source ./src/migrations -r $(name)
migrate@up:
	sqlx migrate run --source ./src/migrations
migrate@down:
	sqlx migrate revert --source ./src/migrations

install:
	apt-get install libpq-dev
	#cargo install diesel_cli --no-default-features --features postgres
	cargo add actix-web
	cargo add actix-cors
	cargo add serde --features derive
	cargo add serde_json
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo install cargo-update
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	# HotReload
	cargo install cargo-watch
	# SQLX-CLI
	cargo install sqlx-cli --no-default-features --features rustls,postgres