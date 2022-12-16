start:
	docker-compose up -d
	docker-compose logs -f

stop:
	docker-compose down --remove-orphans

restart:
	make stop
	make start

logs:
	docker-compose logs -f

website:
	docker-compose run --rm website sh

.PHONY: start stop restart website
