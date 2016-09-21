
docker-setup: 
	docker-machine create --driver virtualbox default

docker-start:
	#docker-machine stop default
	docker-machine start default
	eval "$(docker-machine env default)"

docker-build:
	docker build -t benbrunton/pusoy_dos .

docker-run:
	docker run -v /Users/IWC-BenB/personal/pd_server/project:/project -it benbrunton/pusoy_dos 
