migrate-up:
	diesel migration run

migrate-down:
	diesel migration redo

migrate-create:
	diesel migration generate $(filter-out $@,$(MAKECMDGOALS))

release-dev:
	heroku container:login
	heroku container:push worker --app sleepy-woodland-35890
	heroku container:release worker --app sleepy-woodland-35890

release:
	heroku container:login
	heroku container:push worker --app onepunchman-parcer-bot
	heroku container:release worker --app onepunchman-parcer-bot

#fix-pg-connection:
#    heroku pg:killall --app sleepy-woodland-35890