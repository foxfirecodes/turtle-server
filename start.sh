#!/bin/bash
env UID="$(id -u)" GID="$(id -g)" docker compose up
