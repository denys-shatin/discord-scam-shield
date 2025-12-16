#!/bin/bash
# Быстрый деплой на VPS одной командой

set -e

echo "🚀 Discord Scam Bot - Быстрый деплой"
echo "===================================="
echo ""

# Проверка .env
if [ ! -f ".env" ]; then
    echo "❌ Файл .env не найден!"
    echo ""
    echo "Создай .env файл:"
    echo "  DISCORD_TOKEN=твой_токен"
    echo "  ADMIN_USER_ID=твой_id"
    echo ""
    exit 1
fi

# Проверка Docker
if ! command -v docker &> /dev/null; then
    echo "🐳 Docker не найден. Устанавливаю..."
    curl -fsSL https://get.docker.com | sh
    echo "✅ Docker установлен"
fi

# Проверка Docker Compose
if ! docker compose version &> /dev/null; then
    echo "📦 Устанавливаю Docker Compose..."
    apt-get update
    apt-get install -y docker-compose-plugin
    echo "✅ Docker Compose установлен"
fi

# Остановка старой версии
if docker ps -a | grep -q discord-scam-bot; then
    echo "🛑 Останавливаю старую версию..."
    docker-compose down
fi

# Сборка и запуск
echo "🔨 Собираю и запускаю бота..."
docker-compose up -d --build

echo ""
echo "✅ Готово!"
echo ""
echo "Команды:"
echo "  Логи:        docker-compose logs -f"
echo "  Остановить:  docker-compose down"
echo "  Перезапуск:  docker-compose restart"
echo "  Статус:      docker-compose ps"
echo ""
