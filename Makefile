db:
	docker-compose up -d

stop:
	@docker ps -aq | xargs -r docker rm -f