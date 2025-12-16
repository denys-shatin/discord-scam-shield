# Многоступенчатая сборка для минимального размера
FROM rust:1.75-slim as builder

WORKDIR /app

# Копируем манифесты
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Копируем исходники
COPY src ./src

# Собираем релиз
RUN cargo build --release

# Финальный образ
FROM debian:bookworm-slim

# Устанавливаем только необходимые зависимости
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем бинарник из builder
COPY --from=builder /app/target/release/discord-scam-detector /app/bot

# Создаём непривилегированного пользователя
RUN useradd -m -u 1000 botuser && chown -R botuser:botuser /app
USER botuser

# Запускаем бота
CMD ["/app/bot"]
