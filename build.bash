#!/bin/bash

TAG="travishaas/pinger:latest"

docker build --platform linux/amd64 --tag $TAG .
docker push $TAG
