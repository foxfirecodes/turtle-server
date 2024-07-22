#!/bin/bash
if [ -f .env ]; then
  echo ".env already exists, exiting..."
  exit
fi

touch .env
echo "UID=$(id -u)" >> .env
echo "GID=$(id -g)" >> .env
