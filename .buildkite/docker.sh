#!/bin/bash

MYSQL_ROOT_PASSWORD=prisma
MSSQL_SA_PASSWORD="<YourStrong@Passw0rd>"

docker network create test-net
docker run --name test-postgres --network test-net \
    -e POSTGRES_PASSWORD=prisma \
    -e POSTGRES_USER=prisma \
    -e POSTGRES_DB=prisma -d postgres

docker run --name test-mysql --network test-net \
    -e MYSQL_USER=prisma \
    -e MYSQL_DATABASE=prisma \
    -e MYSQL_ROOT_PASSWORD=$MYSQL_ROOT_PASSWORD \
    -e MYSQL_PASSWORD=prisma -d mysql

docker run --name test-mssql --network test-net \
    -e ACCEPT_EULA=Y \
    -e SA_PASSWORD=$MSSQL_SA_PASSWORD \
    -d mcr.microsoft.com/mssql/server:2019-latest

docker run -w /build --network test-net -v $BUILDKITE_BUILD_CHECKOUT_PATH:/build \
    -e TEST_MYSQL=mysql://prisma:prisma@test-mysql:3306/prisma \
    -e TEST_PSQL=postgres://prisma:prisma@test-postgres:5432/prisma \
    -e TEST_MSSQL="sqlserver://test-mssql:1433;user=SA;password=$MSSQL_SA_PASSWORD;trustServerCertificate=true" \
    prismagraphql/build:test cargo test --features full,json-1,uuid-0_8,chrono-0_4,tracing-log,serde-support

exit_code=$?

docker stop test-postgres
docker rm test-postgres

docker stop test-mysql
docker rm test-mysql

docker network rm test-net

exit $exit_code
