migrate-up:
	diesel migration run

migrate-down:
	diesel migration redo

migrate-create:
	diesel migration generate $(filter-out $@,$(MAKECMDGOALS))

release:
	heroku container:login
	heroku container:push worker --app onepunchman-parcer-bot
	heroku container:release worker --app onepunchman-parcer-bot

run-tests:
	cargo test