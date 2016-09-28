
docker-build:
	docker build -t benbrunton/pusoy_dos .

docker-run:
	docker run -v ${PWD}/project:/project -d -p 0.0.0.0:3000:3000 --name pd_server benbrunton/pusoy_dos 

docker-up:
	docker run -v ${PWD}/project:/project -it -p 0.0.0.0:3000:3000 --name pd_server benbrunton/pusoy_dos sh

docker-stop:
	docker stop pd_server

docker-restart:
	docker restart pd_server

docker-rm:
	docker rm pd_server

sh:
	docker exec -it pd_server sh

edit:
	vim Makefile
