use gpui::App;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::RwLock;

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
pub fn init(_cx: &mut App) {
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
    // 清理静态缓存以便下次获取新语言的翻译
    clear_static_cache();
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

/// 静态字符串缓存，用于返回 &'static str
static STATIC_TRANSLATIONS: OnceCell<RwLock<HashMap<String, String>>> = OnceCell::new();

/// 初始化静态翻译缓存
fn init_static_translations() {
    let mut cache = HashMap::new();
    
    // 加载所有语言的翻译到缓存
    let en_translations = load_from_json_static(include_str!("../../assets/locales/en.json"));
    let zh_cn_translations = load_from_json_static(include_str!("../../assets/locales/zh-CN.json"));
    let zh_tw_translations = load_from_json_static(include_str!("../../assets/locales/zh-TW.json"));
    let ja_translations = load_from_json_static(include_str!("../../assets/locales/ja.json"));
    let ko_translations = load_from_json_static(include_str!("../../assets/locales/ko.json"));
    
    for (key, value) in en_translations {
        cache.insert(format!("en:{}", key), value);
    }
    for (key, value) in zh_cn_translations {
        cache.insert(format!("zh-CN:{}", key), value);
    }
    for (key, value) in zh_tw_translations {
        cache.insert(format!("zh-TW:{}", key), value);
    }
    for (key, value) in ja_translations {
        cache.insert(format!("ja:{}", key), value);
    }
    for (key, value) in ko_translations {
        cache.insert(format!("ko:{}", key), value);
    }
    
    STATIC_TRANSLATIONS.set(RwLock::new(cache)).ok();
}

/// 加载静态翻译
fn load_from_json_static(json: &str) -> HashMap<String, String> {
    serde_json::from_str(json).unwrap_or_default()
}

/// 翻译函数 - 返回 &'static str，用于需要静态字符串的场景
/// 注意：这个函数会返回缓存的字符串引用，所以字符串内容不会改变直到语言切换
pub fn t_static(key: &str) -> &'static str {
    // 确保静态缓存已初始化
    if STATIC_TRANSLATIONS.get().is_none() {
        init_static_translations();
    }
    
    let cache = STATIC_TRANSLATIONS.get().unwrap();
    
    // 获取当前语言的键
    let lang_key = format!("{}:{}", get_language().as_str(), key);
    
    // 优先查找带语言前缀的键
    if let Ok(cache) = cache.read() {
        if let Some(value) = cache.get(&lang_key) {
            // 这是一个 hack，但为了返回 &'static str，我们需要确保字符串在静态内存中
            // 在实际使用中，我们应该重构 trait 定义来返回 String
            // 临时方案：使用 Box::leak（注意：这会造成内存泄漏，仅用于演示）
            // 更好的方案是修改 trait 定义
            return Box::leak(value.clone().into_boxed_str());
        }
        
        // 回退到原始键
        if let Some(value) = cache.get(key) {
            return Box::leak(value.clone().into_boxed_str());
        }
    }
    
    // 如果找不到翻译，返回键本身
    Box::leak(key.to_string().into_boxed_str())
}

/// 清理静态翻译缓存（当语言改变时调用）
pub fn clear_static_cache() {
    if let Some(cache) = STATIC_TRANSLATIONS.get() {
        let _ = cache.write().map(|mut c| c.clear());
    }
}