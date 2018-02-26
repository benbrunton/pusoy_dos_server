
ready-dev: docker-dev-up 

go: setup-db compile-client run-pdserver

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

restart: kill-pdserver run-pdserver


compile-client:
	docker exec -t pd-dev stylus ./client/styles/pusoydos.styl --out public/css
	docker exec -t pd-dev sh -c "cp ./client/js/* public/js/"

setup-db:
	docker exec -t test-mysql sh /mysql/update_db.sh

demo-data:
	docker exec -t test-mysql sh /mysql/demo-data.sh

build-release:
	docker exec -t pd-dev cargo build --release

reload-chat:
	docker exec -t ws-dev forever restartall


docker-server-build:
	docker build -t benbrunton/pd_server -f ./Dockerfile_run .

docker-server-run:
	docker run -d --name pd_server -P benbrunton/pd_server "./target/debug/pd_server"

docker-release: compile-client build-release docker-server-build

