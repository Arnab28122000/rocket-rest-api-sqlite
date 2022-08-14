local_build:
	docker build -t rocket-rest-api -f Dockerfile .

local_run:
	docker run --rm -it -p 8000:8000 --mount type=bind,source="$(shell pwd)",target=/app --name rocket-rest-api rocket-rest-api cargo watch -x 'run --bin rest-api'

local_stop:
	docker stop rocket-rest-api

cut_production_image:
	./cut_build_push_image.sh