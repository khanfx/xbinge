#!/bin/bash

curl http://localhost:3000/rand
echo

# ------------------------------------------------------------------------

uuid()
{
    xxd -l 16 -p /dev/urandom
}

url="http://localhost:3000/items"

json=$(cat <<EOF
{
    "uuid": "$(uuid)",
    "name": "Hello World"
}
EOF
)

echo Generating request with $json
curl $url -H "Content-Type: application/json" -d "$json"
