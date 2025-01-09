#!/bin/bash

curl http://localhost:3000/rand
echo

# ------------------------------------------------------------------------

uuid()
{
    xxd -l 16 -p /dev/urandom
}

url="http://localhost:3000/schedules"

#    "id": "$(uuid)",
#    "id": "7cd04f425170d03a55211236e834a8f7",
#     "episodes": []

json=$(cat <<EOF
{
    "id": "7cd04f425170d03a55211236e834a8f7",
    "name": "The Expanse"
}
EOF
)

echo Generating request with $json
curl -v $url -H "Content-Type: application/json" -d "$json"
e
cho Updating request with new data
curl -v $url -H "Content-Type: application/json" -d "$json"
