use gpui::App;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// 支持的语言列表
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "zh-CN")]
    SimplifiedChinese,
    #[serde(rename = "zh-TW")]
    TraditionalChinese,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "en" => Some(Language::English),
            "zh-CN" | "zh_cn" | "zh" => Some(Language::SimplifiedChinese),
            "zh-TW" | "zh_tw" => Some(Language::TraditionalChinese),
            "ja" => Some(Language::Japanese),
            "ko" => Some(Language::Korean),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::SimplifiedChinese => "zh-CN",
            Language::TraditionalChinese => "zh-TW",
            Language::Japanese => "ja",
            Language::Korean => "ko",
        }
    }
}

/// 翻译数据
pub type Translations = HashMap<String, String>;

/// 全局翻译管理器
static I18N_MANAGER: OnceCell<Mutex<I18nManager>> = OnceCell::new();

#[derive(Debug)]
pub struct I18nManager {
    current_language: Language,
    translations: HashMap<Language, Translations>,
}

impl I18nManager {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
    
        // 加载所有语言的翻译
        for lang in [
            Language::English,
            Language::SimplifiedChinese,
            Language::TraditionalChinese,
            Language::Japanese,
            Language::Korean,
        ] {
            translations.insert(lang, Self::load_translations(lang));
        }
    
        Self {
            current_language: Language::SimplifiedChinese,
            translations,
        }
    }
    fn load_translations(lang: Language) -> Translations {
        match lang {
            Language::English => Self::load_from_json(include_str!("../../assets/locales/en.json")),
            Language::SimplifiedChinese => Self::load_from_json(include_str!("../../assets/locales/zh-CN.json")),
            Language::TraditionalChinese => Self::load_from_json(include_str!("../../assets/locales/zh-TW.json")),
            Language::Japanese => Self::load_from_json(include_str!("../../assets/locales/ja.json")),
            Language::Korean => Self::load_from_json(include_str!("../../assets/locales/ko.json")),
        }
    }

    fn load_from_json(json: &str) -> Translations {
        serde_json::from_str(json).unwrap_or_default()
    }

    pub fn set_language(&mut self, lang: Language) {
        self.current_language = lang;
    }

    pub fn get_language(&self) -> Language {
        self.current_language
    }

    pub fn translate(&self, key: &str) -> String {
        self.translations
            .get(&self.current_language)
            .and_then(|trans| trans.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    pub fn translate_with_args(&self, key: &str, args: &[&str]) -> String {
        let mut result = self.translate(key);
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    }
}

/// 初始化 i18n 系统
pub fn init(cx: &mut App) {
    let manager = I18nManager::new();
    I18N_MANAGER.set(Mutex::new(manager)).unwrap();
}

/// 设置当前语言
pub fn set_language(lang: Language) {
    if let Some(manager) = I18N_MANAGER.get() {
        if let Ok(mut m) = manager.lock() {
            m.set_language(lang);
        }
    }
}

/// 获取当前语言
pub fn get_language() -> Language {
    I18N_MANAGER
        .get()
        .and_then(|m| m.lock().ok())
        .map(|m| m.get_language())
        .unwrap_or(Language::English)
}

/// 翻译函数 - 简单版本
pub fn t(key: &str) -> String {
    I18N_MANAGER
        .get()
        .and_then(|m| m.lock().ok())
        .map(|m| m.translate(key))
        .unwrap_or_else(|| key.to_string())
}

/// 翻译函数 - 带参数版本
pub fn t_args(key: &str, args: &[&str]) -> String {
    I18N_MANAGER
        .get()
        .and_then(|m| m.lock().ok())
        .map(|m| m.translate_with_args(key, args))
        .unwrap_or_else(|| key.to_string())
}

/// 宏版本 - 更简洁的使用方式
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::t($key)
    };
    ($key:expr, $($arg:expr),*) => {
        $crate::t_args($key, &[$($arg),*])
    };
}