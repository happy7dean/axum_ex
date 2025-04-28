#!/bin/bash

# Docker Compose로 Oracle 데이터베이스 시작
docker-compose up -d

# Oracle이 준비될 때까지 대기
echo "Waiting for Oracle to be ready..."
while ! docker-compose exec oracle sqlplus -L system/oracle@//localhost:1521/XE "SELECT 1 FROM DUAL" > /dev/null 2>&1; do
    sleep 5
done
echo "Oracle is ready!"

# 환경 변수 설정
export ORACLE_HOST=localhost
export ORACLE_PORT=1521
export ORACLE_SERVICE=XE
export ORACLE_USER=system
export ORACLE_PASSWORD=oracle

# 테스트 실행
cargo test oracle_integration_test -- --nocapture

# Docker Compose 정리
docker-compose down 