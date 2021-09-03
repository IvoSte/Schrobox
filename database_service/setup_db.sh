#!/bin/bash

# Actual setup script
mongo <<EOF
use admin
db.createUser({
  user:  "$DB_USER",
  pwd: "$DB_PASSWORD",
  roles: [{
    role: "readWrite",
    db: "$DB_DATABASE"
  }]
})

db.auth("$DB_USER", "$DB_PASSWORD")

use $DB_DATABASE

db.createCollection("$DB_REFRESH_COLLECTION");
db.$DB_REFRESH_COLLECTION.createIndex({ "createdAt": 1 }, { expireAfterSeconds: $JWT_REFRESH_EXPIRE_TIME });

db.createCollection("$DB_VERIFICATION_COLLECTION");
db.$DB_VERIFICATION_COLLECTION.createIndex({ "createdAt": 1 }, { expireAfterSeconds: $VERIFICATION_EXPIRE_TIME });

db.createCollection("$DB_PASSWORD_RESET_COLLECTION");
db.$DB_PASSWORD_RESET_COLLECTION.createIndex({ "createdAt": 1 }, { expireAfterSeconds: $PASSWORD_RESET_EXPIRE_TIME });

db.createCollection("$DB_USER_COLLECTION");

EOF
