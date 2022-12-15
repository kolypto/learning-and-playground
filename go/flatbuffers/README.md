# Flatbuffers

Main feature: memory efficient. Reads directly from the message. No parsing step required.

Define schema: `monster.fbs`. Then use

> $ flatc -o protoc --go proto/monster.fbs

