.PHONY: help
help:
	@echo "Available scripts:"
	@echo "	setup: create the needed files to build the image"
	@echo "	build: build the dockerfile"
	@echo "	run: run the container"

.PHONY: setup
setup:
	touch ./config/.ssh/authorized_keys

.PHONY: build
build:
	make setup
	podman build -t seeker .

.PHONY: run
run:
	podman run -it -v ./config/.ssh:/repo/.ssh -p 2222:22 seeker
