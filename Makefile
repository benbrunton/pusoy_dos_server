
ready-dev: setup docker-dev-up 

go: setup-db compile-client run-pdserver

setup:
	docker network rm pd
	docker network create pd

# dev server

docker-dev-build:
	docker-compose -f docker-services/dev.yml -p pd_dev build

docker-dev-up:
	docker-compose -f docker-services/dev.yml -p pd_dev up -d

docker-dev-down:
	docker-compose -f docker-services/dev.yml -p pd_dev down

run-pdserver:
	docker exec -t pd-dev cargo run

kill-pdserver:
	docker exec -t pd-dev killall pd_server


compile-client:
	docker exec -t pd-dev stylus ./client/styles/pusoydos.styl --out public/css
	docker exec -t pd-dev sh -c "cp ./client/js/* public/js/"

setup-db:
	docker exec -t test-mysql sh /mysql/update_db.sh


docker-server-build:
	docker build -t benbrunton/pd_server -f ./Dockerfile_run .

docker-server-run:
	docker run -d --name pd_server -P benbrunton/pd_server "./target/debug/pd_server"

docker-release: 
	cd project && make css js
	docker exec -t pd_server cargo build --release

