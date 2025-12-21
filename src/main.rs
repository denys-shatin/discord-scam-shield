mod scam_detector;
mod ocr;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler {
    detector: scam_detector::ScamDetector,
    admin_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Игнорируем ботов
        if msg.author.bot {
            return;
        }

        // Игнорируем ЛС (только серверные сообщения)
        if msg.guild_id.is_none() {
            return;
        }

        println!("📨 Сообщение от {}: {}", msg.author.tag(), msg.content);

        let mut content = msg.content.clone();
        
        // Проверка изображений
        for attachment in &msg.attachments {
            if attachment.width.is_some() {
                println!("🖼️ Анализирую картинку: {}", attachment.filename);
                
                match reqwest::get(&attachment.url).await {
                    Ok(img_bytes) => {
                        println!("📥 Картинка загружена");
                        match img_bytes.bytes().await {
                            Ok(bytes) => {
                                println!("🔍 Запускаю OCR...");
                                match ocr::extract_text(&bytes).await {
                                    Ok(text) => {
                                        if !text.is_empty() {
                                            println!("📝 Текст из картинки: {}", text);
                                            content.push_str(&format!(" [IMG: {}]", text));
                                        } else {
                                            println!("⚠️ Текст не найден в картинке");
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Ошибка OCR: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Ошибка чтения байтов: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ Ошибка загрузки картинки: {}", e);
                    }
                }
            }
        }

        if content.is_empty() {
            return;
        }

        // Анализ на скам
        match self.detector.is_scam(&content).await {
            Ok(true) => {
                println!("🚨 СКАМ ОБНАРУЖЕН! Удаляю сообщение...");
                
                // Удаляем сообщение
                if let Err(e) = msg.delete(&ctx.http).await {
                    eprintln!("❌ Ошибка удаления: {}", e);
                    return;
                }
                
                println!("✅ Сообщение удалено");

                // Отправляем админу
                if let Ok(admin) = serenity::model::id::UserId::new(self.admin_id)
                    .to_user(&ctx.http)
                    .await
                {
                    use serenity::builder::{CreateMessage, CreateEmbed};
                    use serenity::model::Colour;
                    
                    let embed = CreateEmbed::new()
                        .title("🚨 Обнаружен и удалён СКАМ")
                        .colour(Colour::RED)
                        .field("👤 Пользователь", format!("{} (ID: {})", msg.author.tag(), msg.author.id), false)
                        .field("📍 Канал", format!("<#{}>", msg.channel_id), true)
                        .field("🕐 Время", format!("<t:{}:F>", msg.timestamp.unix_timestamp()), true)
                        .field("💬 Сообщение", 
                            if content.len() > 1000 {
                                format!("{}...", content.chars().take(1000).collect::<String>())
                            } else {
                                content.clone()
                            }, 
                            false
                        )
                        .footer(serenity::builder::CreateEmbedFooter::new("Антискам бот"))
                        .timestamp(msg.timestamp);
                    
                    let dm = admin
                        .direct_message(&ctx.http, CreateMessage::new().embed(embed))
                        .await;
                    
                    if let Err(e) = dm {
                        eprintln!("Не удалось отправить ЛС админу: {}", e);
                    }
                }
            }
            Ok(false) => {
                println!("✓ Сообщение чистое");
            }
            Err(e) => eprintln!("❌ Ошибка детектора: {}", e),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ {} подключен!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Загружаем .env файл
    dotenv::dotenv().ok();
    
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN не найден");
    let admin_id: u64 = env::var("ADMIN_USER_ID")
        .expect("ADMIN_USER_ID не найден")
        .parse()
        .expect("ADMIN_USER_ID должен быть числом");

    let detector = scam_detector::ScamDetector::new()
        .await
        .expect("Не удалось загрузить модель");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { detector, admin_id })
        .await
        .expect("Ошибка создания клиента");

    if let Err(e) = client.start().await {
        eprintln!("Ошибка клиента: {:?}", e);
    }
}
