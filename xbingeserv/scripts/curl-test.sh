#!/bin/bash

curl http://localhost:3000/rand
echo

# ------------------------------------------------------------------------

uuid()
{
    xxd -l 16 -p /dev/urandom
}

url="http://localhost:3000/schedules"

json=$(cat <<EOF
{
    "id": "$(uuid)",
    "name": "The Expanse",
    "episodes": []
}
EOF
)

echo Generating request with $json
curl -v $url -H "Content-Type: application/json" -d "$json"
