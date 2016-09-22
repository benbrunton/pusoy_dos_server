
docker-setup: 
	docker-machine create --driver virtualbox default

docker-start:
	docker-machine start default
	eval "$(docker-machine env default)"

docker-build:
	docker build -t benbrunton/pusoy_dos .

docker-run:
	docker run -v ${PWD}/project:/project -d -p 0.0.0.0:3000:3000 benbrunton/pusoy_dos 

edit:
	vim Makefile
