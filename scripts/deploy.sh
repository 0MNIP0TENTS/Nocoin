#!/bin/bash

# Pull the latest Docker image for the consensus nodes
docker pull nocoin/nocoin_node:latest

# Launch the network using Docker Compose
docker-compose up -d

# Initialize network settings
docker exec nocoin_node_1 ./initialize_network.sh
