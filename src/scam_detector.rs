use anyhow::Result;

pub struct ScamDetector {
    patterns: Vec<ScamPattern>,
}

struct ScamPattern {
    keywords: Vec<&'static str>,
}

impl ScamDetector {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            patterns: vec![
                ScamPattern {
                    keywords: vec![
                        // Крипто
                        "airdrop", "free crypto", "claim", "giveaway", "prize", "winner",
                        "congratulations", "verify", "wallet", "metamask", "trust wallet",
                        "connect wallet", "seed phrase", "private key", "recovery phrase",
                        "crypto", "token", "nft", "usdt", "eth", "btc",
                        
                        // Деньги и заработок
                        "$", "usd", "dollar", "money", "cash", "profit", "income",
                        "paid", "payment", "investment",
                        
                        // Срочность и призывы
                        "urgent", "limited time", "act now", "click here", "dm me",
                        "interested", "reply with", "check out", "link in bio",
                        
                        // Подозрительные фразы
                        "not a scam", "not spam", "no scam", "legit", "guaranteed",
                        "course", "step-by-step", "insanely well",
                        
                        // Русские
                        "раздача", "бесплатно", "получи", "выиграл", "приз",
                        "кошелек", "крипта", "биткоин", "эфир", "токен", "нфт",
                        "инвестиция", "прибыль", "гарантия", "удвоить", "заработок",
                        "курс", "пошагово",
                    ],
                },
            ],
        })
    }

    pub async fn is_scam(&self, text: &str) -> Result<bool> {
        let text_lower = text.to_lowercase();
        
        println!("🔍 Анализирую: {}", text_lower);
        
        let mut score = 0;
        let mut has_crypto = false;
        let mut has_money = false;

        // Проверка ключевых слов
        for pattern in &self.patterns {
            let mut keyword_matches = 0;
            
            for keyword in &pattern.keywords {
                if text_lower.contains(keyword) {
                    keyword_matches += 1;
                    
                    // Проверка крипто-терминов
                    if ["crypto", "btc", "eth", "usdt", "wallet", "metamask", "крипта", "биткоин", "кошелек", "token", "nft"].contains(keyword) {
                        has_crypto = true;
                    }
                    
                    // Проверка денег
                    if ["$", "usd", "dollar", "money", "cash", "profit", "income", "making", "earn", "paid"].contains(keyword) {
                        has_money = true;
                    }
                }
            }
            
            score += keyword_matches;
        }

        // Проверка упоминания денег
        if self.contains_money_amount(&text_lower, 50.0) {
            has_money = true;
            score += 3;
        }

        // Проверка подозрительных ссылок
        if text_lower.contains("http") || text_lower.contains("www.") || text_lower.contains(".com") {
            score += 2;
        }

        // Проверка призывов к действию
        if text_lower.contains("click") || text_lower.contains("жми") || text_lower.contains("перейди") {
            score += 2;
        }

        // Скам если: много ключевых слов ИЛИ (крипта/деньги + призывы)
        let is_scam = score >= 5 || (score >= 3 && (has_crypto || has_money));
        
        println!("📊 Score: {}, Crypto: {}, Money: {}, Scam: {}", score, has_crypto, has_money, is_scam);
        
        Ok(is_scam)
    }

    fn contains_money_amount(&self, text: &str, threshold: f64) -> bool {
        let patterns = [
            r"\$\s*\d+", r"\d+\s*\$", r"\d+\s*usd", r"\d+\s*usdt",
            r"\d+\s*долларов", r"\d+\s*баксов",
        ];
        
        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(cap) = re.find(text) {
                    let num_str: String = cap.as_str()
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect();
                    
                    if let Ok(amount) = num_str.parse::<f64>() {
                        if amount >= threshold {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
}
