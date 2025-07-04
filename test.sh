#!/bin/bash

BASE_URL="http://localhost:3000"
COOKIE_JAR="cookies.txt"

# Nettoyage
rm -f "$COOKIE_JAR"

EMAIL="test@example.com"
PASSWORD="supersecure"

echo "🔐 Register..."
curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -c "$COOKIE_JAR" \
  -d '{
    "email": "'"$EMAIL"'",
    "password": "'"$PASSWORD"'"
  }' | jq

echo -e "\n✅ Register terminé."

echo "🔐 Login..."
curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -b "$COOKIE_JAR" -c "$COOKIE_JAR" \
  -d '{
    "email": "'"$EMAIL"'",
    "password": "'"$PASSWORD"'"
  }' | jq

echo -e "\n✅ Login terminé."

echo "🔄 Refresh token..."
curl -s -X GET "$BASE_URL/api/auth/refresh" \
  -b "$COOKIE_JAR" -c "$COOKIE_JAR" | jq

echo -e "\n✅ Refresh terminé."

echo "🚪 Logout..."
curl -s -X POST "$BASE_URL/api/auth/logout" \
  -b "$COOKIE_JAR" -c "$COOKIE_JAR" | jq

echo -e "\n✅ Logout terminé."
