#!/bin/bash
# Проверка что секреты не попадут в git

echo "🔍 Проверка секретов перед коммитом..."

# Проверка .env
if git ls-files --error-unmatch .env 2>/dev/null; then
    echo "❌ ОШИБКА: .env в git! Удали его:"
    echo "   git rm --cached .env"
    exit 1
fi

# Проверка токенов в коде
if git grep -i "discord_token.*=" -- '*.rs' '*.toml' | grep -v "env::var"; then
    echo "❌ ОШИБКА: Найдены токены в коде!"
    exit 1
fi

# Проверка .gitignore
if ! grep -q "^.env$" .gitignore; then
    echo "❌ ОШИБКА: .env не в .gitignore!"
    exit 1
fi

echo "✅ Всё чисто! Можно коммитить."
