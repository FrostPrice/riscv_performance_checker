#!/bin/sh

date=$(date)
echo $date

echo Starting Riscv Performance Checker - API

# Clear file if exists
echo "#Starting env file" >.env

# Add Host config to .env file
if [ -z ${APP_HOST} ]; then
    echo Host not defined, using default: 0.0.0.0
    echo APP_HOST=0.0.0.0 >>.env
else
    echo Host: $APP_HOST
    echo APP_HOST=$APP_HOST >>.env
fi

# Add Port config to .env file
if [ -z ${APP_PORT} ]; then
    echo Port not defined, using default: 80
    echo APP_PORT=80 >>.env
else
    echo Port: $APP_PORT
    echo APP_PORT=$APP_PORT >>.env
fi

# Add Database config to .env file
if [ -z ${DATABASE_URL} ]; then
    echo Fatal error: DATABASE_URL not defined
    exit 1
else
    echo Database URL: $DATABASE_URL
    echo DATABASE_URL=$DATABASE_URL >>.env
fi

exec ./riscv_performance_checker

