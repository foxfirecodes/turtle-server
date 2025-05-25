#!/bin/bash

grep modrinth.com docker-compose.yml | \
  cut -d / -f 5 | \
  sed 's;^;https://modrinth.com/mod/;g' | \
  sed 's;$;/versions?l=fabric\&g=1.21.5;g'
