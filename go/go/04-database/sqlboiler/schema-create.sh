#! /usr/bin/env bash 

cat schema.sql | psql postgres://postgres:postgres@localhost:5432/postgres 
