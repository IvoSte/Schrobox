
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

db.createCollection("$DB_LEDGER_COLLECTION");
db.$DB_LEDGER_COLLECTION.createIndex({"groupId": 1}, true);
db.$DB_LEDGER_COLLECTION.createIndex({"entries.entryId": 1}, { "background": true, "sparse": true });

db.createCollection("$DB_ANNOUNCE_COLLECTION");
db.$DB_ANNOUNCE_COLLECTION.createIndex({"groupId": 1}, true);
db.$DB_ANNOUNCE_COLLECTION.createIndex({"announcements.announcementId": 1}, { "background": true, "sparse": true });

db.createCollection("$DB_REFRESH_COLLECTION");
db.$DB_REFRESH_COLLECTION.createIndex({ "createdAt": 1 }, { expireAfterSeconds: $JWT_REFRESH_EXPIRE_TIME });

db.createCollection("$DB_VERIFICATION_COLLECTION");
db.$DB_VERIFICATION_COLLECTION.createIndex({ "createdAt": 1 }, { expireAfterSeconds: $VERIFICATION_EXPIRE_TIME });

db.createCollection("$DB_PASSWORD_RESET_COLLECTION");
db.$DB_PASSWORD_RESET_COLLECTION.createIndex({ "createdAt": 1 }, { expireAfterSeconds: $PASSWORD_RESET_EXPIRE_TIME });

db.createCollection("$DB_GROUP_COLLECTION");
db.createCollection("$DB_USER_COLLECTION");
db.createCollection("$DB_ACTIVITY_COLLECTION");

EOF
