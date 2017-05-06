
docker-build-dev:
	docker build -t benbrunton/pusoy_dos .

docker-run-dev: docker-stop-dev
	docker rm pd_dev
	docker run --name pd_dev -d -v ${PWD}/project:/project benbrunton/pusoy_dos

docker-restart-dev:
	docker restart pd_dev

docker-stop-dev:
	docker stop pd_dev

run-pdserver:
	docker exec -t pd_dev cargo run

compile-client:
	docker exec -t pd_dev stylus ./client/styles/pusoydos.styl --out public/css
	docker exec -t pd_dev sh -c "cp ./client/js/* public/js/"


docker-server-build:
	docker build -t benbrunton/pd_server -f ./Dockerfile_run .

docker-server-run:
	docker run -d --name pd_server -P benbrunton/pd_server "./target/debug/pd_server"

docker-release: 
	cd project && make css js
	docker exec -t pd_server cargo build --release

docker-stop:
	docker stop pd_server

docker-restart:
	docker restart pd_server

docker-rm:
	docker rm pd_server

sh:
	docker exec -it pd_server bash

tail-logs:
	docker logs -f pd_server

reset-db:
	docker exec pd_server ./scripts/reset-db

deploy: 
	sh .deploy
