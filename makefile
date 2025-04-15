build-database:
	podman pull postgres:latest
	podman run --name quests-tracker-db -e POSTGRES_PASSWORD=123456 -e POSTGRES_DB=quests_tracker_db -p 5432:5432 -d postgres:latest

migrate-up:
	diesel migration run

migrate-down:
	diesel migration revert

build-app:
	podman build -t quests-tracker:v1.0.0 .

# Edit your .env file to set the correct database URL and JWT secrets before running this command.
start-container:
	podman run --name quests-tracker -p 8080:8080 \
		-e STAGE=Local \
		-e SERVER_PORT=8080 \
		-e SERVER_BODY_LIMIT=10 \
		-e SERVER_TIMEOUT=90 \
		-e DATABASE_URL=postgres://postgres:123456@localhost:5432/quests_tracker_db \
		-e JWT_ADVENTURER_SECRET=xxxxx \
		-e JWT_ADVENTURER_REFRESH_SECRET=xxxxx \
		-e JWT_GUILD_COMMANDER_SECRET=xxxxx \
		-e JWT_GUILD_COMMANDER_REFRESH_SECRET=xxxxx \
		-d quests-tracker:v1.0.0

tests:
	cargo tarpaulin --out html