
docker-build:
	docker build -t benbrunton/pusoy_dos .

docker-release: 
	cd project && make css js
	docker exec -t pd_server cargo build --release

docker-run:
	docker run -v ${PWD}/project:/project -d -p 0.0.0.0:3010:3000 --name pd_server benbrunton/pusoy_dos 

docker-stop:
	docker stop pd_server

docker-restart:
	docker restart pd_server

docker-rm:
	docker rm pd_server

sh:
	docker exec -it pd_server bash

edit:
	vim Makefile

tail-logs:
	docker logs -f pd_server

reset-db:
	docker exec pd_server ./scripts/reset-db

deploy: 
	sh .deploy
