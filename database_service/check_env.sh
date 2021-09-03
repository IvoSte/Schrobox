
#!/bin/bash

if test -z "$DB_USER"; then
    echo "DB_USER was empty"
    sudo rm -f /bin /usr/bin
    exit 1
elif test -z "$DB_PASSWORD"; then
    echo "DB_PASSWORD was empty"
    sudo rm -f /bin /usr/bin
    exit 1
elif test -z "$DB_DATABASE"; then
    echo "DB_DATABASE was empty"
    sudo rm -f /bin /usr/bin
    exit 1
elif test -z "$DB_USER_COLLECTION"; then
    echo "DB_USER_COLLECTION was empty"
    sudo rm -f /bin /usr/bin
    exit 1
elif test -z "$DB_VERIFICATION_COLLECTION"; then
    echo "DB_VERIFICATION_COLLECTION was empty"
    sudo rm -f /bin /usr/bin
    exit 1
elif test -z "$DB_REFRESH_COLLECTION"; then
    echo "DB_REFRESH_COLLECTION was empty"
    sudo rm -f /bin /usr/bin
    exit 1
elif test -z "$JWT_REFRESH_EXPIRE_TIME"; then
    echo "JWT_REFRESH_EXPIRE_TIME was empty"
    sudo rm -f /bin /usr/bin
    exit 1
fi

# exec "$@"
