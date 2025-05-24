use std::env;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
    role: String,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentText,
}

#[derive(Deserialize)]
struct ContentText {
    parts: Vec<PartText>,
}

#[derive(Deserialize)]
struct PartText {
    text: String,
}

pub fn build_prompt(diff_or_list: &str, lang: &str) -> String {
    match lang {
        "ru" => format!(
            r#"
            Ты — AI для генерации сообщений коммитов.

На входе у тебя — список изменений в проекте в формате diff или список изменённых файлов.
Твоя задача — сгенерировать короткое, информативное и понятное **сообщение коммита**, которое точно отражает внесённые изменения.

**Важно:**
- Выводи **только** текст сообщения коммита — никаких объяснений, приветствий, метаданных, кавычек и форматирования.
- Используй повелительное наклонение (например: «Добавить…», «Исправить…», «Удалить…»).
- Сообщение должно быть кратким, но ёмким (рекомендуется от 50 до 72 символов в заголовке).
- При необходимости можешь добавить короткое тело (одна-две строки), отделённое пустой строкой, где объяснишь причину или контекст изменений.
- Никакой дополнительной информации, только сообщение коммита.

Вот список изменений:
{}"#,
            diff_or_list
        ),
        _ => format!(
            r#"You are a commit message generator AI.

Given the following diff or list of changed files, generate a concise, clear, and informative **git commit message** that summarizes the changes.
The message should be suitable for use in professional software projects.

**Important:**
- Output **only** the commit message text — no explanations, no greetings, no metadata, no markdown formatting, no quotes.
- Use the imperative mood (e.g., "Add feature", "Fix bug").
- Keep it concise but descriptive (ideally 50-72 characters in the summary line).
- If needed, you may add a short body (one or two sentences) explaining the why or context of the changes, separated from the summary by a blank line.
- Do not include any other information besides the commit message.

Here are the changes:
{}"#,
            diff_or_list
        ),
    }
}

pub async fn get_commit_message(prompt: &str) -> anyhow::Result<String> {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY")?;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let body = GeminiRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let client = reqwest::Client::new();
    let res = client.post(&url).json(&body).send().await?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        anyhow::bail!("Request failed ({}): {}", status, text);
    }

    let response: GeminiResponse = res.json().await?;
    let msg = response
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.trim().to_string())
        .unwrap_or_else(|| "Generated commit message".to_string());

    Ok(msg)
}
