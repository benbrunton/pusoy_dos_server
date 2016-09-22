
docker-setup: 
	docker-machine create --driver virtualbox default

docker-start:
	docker-machine start default
	eval "$(docker-machine env default)"

docker-build:
	docker build -t benbrunton/pusoy_dos .

docker-run:
	docker run -v ${PWD}/project:/project -t benbrunton/pusoy_dos 

edit:
	vim Makefile
