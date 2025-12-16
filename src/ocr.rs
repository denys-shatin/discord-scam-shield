use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};

// OCR.space API - бесплатный, быстрый OCR
pub async fn extract_text(image_bytes: &[u8]) -> Result<String> {
    let api_key = std::env::var("OCR_SPACE_API_KEY")
        .expect("OCR_SPACE_API_KEY не найден в .env! Получи ключ на https://ocr.space/ocrapi");
    
    println!("📦 Кодирую картинку...");
    let base64_image = general_purpose::STANDARD.encode(image_bytes);
    
    println!("🌐 Отправляю в OCR.space...");
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://api.ocr.space/parse/image")
        .header("apikey", api_key)
        .form(&[
            ("base64Image", format!("data:image/png;base64,{}", base64_image).as_str()),
            ("language", "eng"),
            ("isOverlayRequired", "false"),
            ("detectOrientation", "true"),
            ("scale", "true"),
            ("OCREngine", "2"), // Движок 2 - быстрее и точнее
        ])
        .send()
        .await?;
    
    println!("📨 Ответ получен: {}", response.status());
    
    let result: serde_json::Value = response.json().await?;
    
    // Проверяем ошибки
    if let Some(error) = result["ErrorMessage"].as_str() {
        if !error.is_empty() {
            eprintln!("❌ Ошибка OCR: {}", error);
            return Ok(String::new());
        }
    }
    
    // Извлекаем текст
    let text = result["ParsedResults"][0]["ParsedText"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    println!("✅ OCR завершен, найдено {} символов", text.len());
    
    Ok(text.trim().to_string())
}
