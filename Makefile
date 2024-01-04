# Include .env file if it exists
-include .env

# Define variables
# .env imported variables
PORT := $(if $(PORT),$(PORT), 3001)
DOCKER_INTERNAL_PORT := $(if $(DOCKER_INTERNAL_PORT),$(DOCKER_INTERNAL_PORT), 3001)
DOCKER_NAME = $(shell basename $(CURDIR))
DOCKER_IMAGE = $(DOCKER_NAME)_image
DOCKER_CONTAINER = $(DOCKER_IMAGE)_container
DOCKER_PATH = ./utils/dockerfiles/
IMAGE_ID := $(shell docker images --filter "label=com.docker.compose.project=$(DOCKER_IMAGE)" --format "{{.ID}}" | head -n 1)


# ================
# Default target
##@ Docker
.PHONY: buildrun
buildrun: build run ##Build the container first, then run it, development.

.PHONY: run
run: ##Run the Docker container.
	@# PORTS set in the Dockerfile, will be overrriden by the docker-compose.yml file.
	@docker run -d --name $(DOCKER_CONTAINER) $(DOCKER_IMAGE) 
	@echo "Docker container $(DOCKER_CONTAINER) is running."

.PHONY: build
build: ##Build the Docker image, development by default. 
	@docker build -t $(DOCKER_IMAGE) .
	@echo "Docker image $(DOCKER_IMAGE) is built."

.PHONY: stop
stop: ##Stops the current running container.
	@docker stop $(DOCKER_CONTAINER)
	@docker rm $(DOCKER_CONTAINER)
	@echo "Docker container $(DOCKER_CONTAINER) is stopped and removed."

.PHONY: clean
clean: ##Remove the Docker image.
	@docker rmi $(DOCKER_IMAGE)
	@echo "Docker image $(DOCKER_IMAGE) is removed."

# View the container's logs
.PHONY: logs
logs: ##Show the logs message.
	@echo "Docker image $(DOCKER_IMAGE) logs:"
	@docker logs ${DOCKER_CONTAINER}

.PHONY: watch_logs
watch_logs: ##Live log messages.
	@echo "Docker image $(DOCKER_IMAGE) logs:"
	@docker logs -f $(DOCKER_CONTAINER)

.PHONE: stats
stats: ##Monitor the docker stats.
	@docker stats

.PHONY: prune
prune: ##Safe docker prune. Clean up space.
	@docker system prune

.PHONY: unsafe_prune
unsafe_prune: ##Forced docker prune without safety prompt. Cleans up more space.
	@docker system prune -af

## ===================
##@ Custom Docker Commands
.PHONY: get_repository_name
get_repository_name: ##Gets the repository name IF the print_image_id exists.
	@docker images --format '{{.ID}} {{.Repository}}' | grep -w "$(IMAGE_ID)" | awk '{printf "%s", $$2}'

.PHONY: composebuild-default
composebuild-default: ##Docker compose build. rustbot. No cache by default.
	docker compose -p $(DOCKER_IMAGE) -f $(DOCKER_PATH)docker-compose.yml build --no-cache 

.PHONY: print_image_id_raw
print_image_id_raw: ##Update the latests image ID.
	@echo -n "$(IMAGE_ID)"

.PHONY: print_image_id
print_image_id: ##Update the latests image ID.
	@echo "Image ID: $(IMAGE_ID)"

.PHONY: stop_matching_containers
stop_matching_containers: ## Stop all running containers with the DOCKER_IMAGE name.
	@echo "Stopping all containers with label com.docker.compose.project=$(DOCKER_IMAGE)"
	@docker ps --filter "label=com.docker.compose.project=$(DOCKER_IMAGE)" --format "{{.ID}}" | xargs -I {} sh -c 'echo "Stopping container: {}" && docker stop {} && docker rm -v {}'
	@echo "Stopping complete"

.PHONY: delete_matching_images
delete_matching_images: ## Delete all local images with the DOCKER_IMAGE name.
	@echo "Deleting images with label com.docker.compose.project=$(DOCKER_IMAGE)"
	@docker images --filter "label=com.docker.compose.project=$(DOCKER_IMAGE)" --format "{{.ID}}" | xargs -I {} sh -c 'echo "Deleting image: {}" && docker rmi {}'
	@echo "Deletion complete"

.PHONY: save_image_as_tar
save_image_as_tar: ## Save the Docker image as a zip file
	@echo "Saving Docker image $(DOCKER_IMAGE) with ID $(IMAGE_ID) as a tar file..."
	docker save -o $(DOCKER_IMAGE).tar $(shell make get_repository_name):latest
	gzip $(DOCKER_IMAGE).tar
	@echo "Docker image saved as $(DOCKER_IMAGE).tar.gz"

.PHONY: echo_docker_image_name
echo_docker_image_name: ## Prints the image name.
	@echo -n $(DOCKER_IMAGE)

.PHONY: echo_foldername
echo_foldername: ## Prints the foldername.
	@echo -n $(DOCKER_NAME)

## ===================
##@ Docker Compose

.PHONY: composedown
composedown:	##Docker compose down.
	@docker compose -p $(DOCKER_IMAGE) down

.PHONY: composerm
composerm:	##Docker compose rm.
	@docker compose -p $(DOCKER_IMAGE) rm

.PHONY: composelist
composelist: ##List all docker services that can be run in this project.
	@ #Leaving echo -n "". There seems to be a makefile error here, on files that do not contain the #MakefileServiceName otherwise.
	@for file in $(DOCKER_PATH)docker-compose.*.yml; do \
		grep -A1 '# MakefileServiceName' "$$file" | grep -v '# MakefileServiceName' | sed -E "s/^([^:]+):/\x1b[31m\1\x1b[0m:/"; \
		echo -n ""; \
	done

.PHONY: composelistfull
composelistfull: ##List all docker services that can be run in this project. With filename.
	@for file in $(DOCKER_PATH)docker-compose.*.yml; do \
		echo "Services in $$file:"; \
		grep -A1 '# MakefileServiceName' "$$file" | grep -v '# MakefileServiceName' | sed -E "s/^([^:]+):/\x1b[31m\1\x1b[0m:/"; \
		echo ""; \
	done

# ================
##@ Label section
.PHONY: help
help:  ##Show this help message.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.PHONY: version
version: ##Show version of `Docker`, `Docker compose`.`
	@ docker -v
	@ docker compose version

.PHONY: systemstats
systemstats: ##Show Diskspace, CPU and RAM.
	@echo "Disk Usage: "
	@df | awk 'NR>1 {size+=$$2; used+=$$3; avail+=$$4} \
		END {cmd_size="echo " size " | numfmt --to=iec --from-unit=1000"; \
		cmd_used="echo " used " | numfmt --to=iec --from-unit=1000"; \
		cmd_avail="echo " avail " | numfmt --to=iec --from-unit=1000"; \
		cmd_size | getline formattedSize; close(cmd_size); \
  	cmd_used | getline formattedUsed; close(cmd_used); \
    cmd_avail | getline formattedAvail; close(cmd_avail); \
    printf "%8s%8s%8s%8s\n", "Size", "Used", "Avail.", "Use%"; \
    printf "%8s%8s%8s%8.2f%%\n", formattedSize, formattedUsed, formattedAvail, (used / size) * 100}'
	@echo "=================================="
	@echo ""
	@echo "Memory: "
	@free -h
	@echo "=================================="
	@echo ""
	@lscpu | grep -v -E "On-line CPU\(s\) list:|NUMA node0 CPU\(s\):" | grep -E "CPU\(s\)|Thread\(s\) per core|Model name|CPU.*MHz|BogoMIPS"

.PHONY: diskspace
diskspace: ##Show detailed disk space information.
	@df -h
