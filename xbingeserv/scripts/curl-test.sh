#!/bin/bash

uuid()
{
    xxd -l 16 -p /dev/urandom
}

id1=$(uuid)
echo $id1

url="http://localhost:3000/items"

json=$(cat <<EOF
{
    "uuid": "$(uuid)"
    "name": "Hello World"
}
EOF
)

echo Generating request with $json
#curl $url -H "Content-Type: application/json" -d "$json"

curl http://localhost:3000/rand
