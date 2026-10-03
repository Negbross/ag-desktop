use serde_json::json;
use ag_desktop::AgentAction;

const SYSTEM_PROMPT: &str = r#"Kamu agent kontrol desktop Windows. Kamu akan diberi histori percakapan + hasil aksi sebelumnya (kalau ada). Tugasmu: putuskan SATU aksi berikutnya yang paling tepat untuk melanjutkan instruksi user.

ATURAN PENTING:
1. Jika target (hwnd/pid) belum diketahui dari histori, panggil list_windows atau list_processes dulu untuk menemukannya. JANGAN PERNAH mengarang angka hwnd/pid.
2. Untuk membuka aplikasi baru (yang belum ada di list_windows), gunakan launch_process dengan nama exe-nya (contoh: "notepad.exe", "calc.exe"). JANGAN gunakan find_file untuk mencari aplikasi yang ingin dijalankan — find_file hanya untuk mencari file dokumen/data, bukan untuk membuka aplikasi.
3. Setelah launch_process, SELALU panggil list_windows di langkah berikutnya untuk menemukan hwnd dari aplikasi yang baru dibuka, baru lanjut focus_window.
4. type_text HANYA menerima teks literal apa adanya (TIDAK ADA syntax hotkey seperti ^n, {ENTER}, atau sejenisnya). Untuk tombol khusus (enter, tab, escape, dll), gunakan key_press SEBAGAI LANGKAH TERPISAH setelah type_text.
5. Sebelum type_text atau key_press ke suatu window, pastikan window tersebut sudah di-focus_window terlebih dahulu di langkah sebelumnya.
6. Jika instruksi user sudah sepenuhnya selesai dikerjakan, balas {"action":"done"}.
7. Balas HANYA satu object JSON, tanpa array, tanpa penjelasan, tanpa markdown code block.

Action yang valid beserta field-nya:
{"action":"done"}
{"action":"list_windows"}
{"action":"list_processes"}
{"action":"launch_process","name":"<string, contoh: notepad.exe>"}
{"action":"kill_process","pid":<int>}
{"action":"focus_window","hwnd":<int>}
{"action":"minimize_window","hwnd":<int>}
{"action":"maximize_window","hwnd":<int>}
{"action":"restore_window","hwnd":<int>}
{"action":"close_window","hwnd":<int>}
{"action":"system_overview"}
{"action":"get_volume"}
{"action":"set_volume","percent":<0-100>}
{"action":"mute"}
{"action":"unmute"}
{"action":"move_mouse","x":<int>,"y":<int>}
{"action":"click","button":"left/right/middle"}
{"action":"scroll","amount":<int>}
{"action":"type_text","text":"<string>"}
{"action":"key_press","key":"enter/tab/escape/backspace/space"}
{"action":"find_file","path":"<string>","pattern":"<string>"}"#;

const ACTION_SCHEMA_SINGLE: &str = r#"{
    "type": "object",
    "properties": {
        "action": { "type": "string" },
        "hwnd": { "type": "integer" },
        "pid": { "type": "integer" },
        "name": { "type": "string" },
        "percent": { "type": "number" },
        "text": { "type": "string" },
        "key": { "type": "string" },
        "path": { "type": "string" },
        "pattern": { "type": "string" },
        "x": { "type": "integer" },
        "y": { "type": "integer" },
        "button": { "type": "string" },
        "amount": { "type": "integer" }
    },
    "required": ["action"]
}"#;

async fn next_action(history: &str) -> anyhow::Result<AgentAction> {
    let client = reqwest::Client::new();
    let schema: serde_json::Value = serde_json::from_str(ACTION_SCHEMA_SINGLE)?;

    let resp = client
        .post("http://localhost:11434/api/chat")
        .json(&json!({
            "model": "qwen2.5:7b-instruct",
            "messages": [
                { "role": "system", "content": SYSTEM_PROMPT },
                { "role": "user", "content": history }
            ],
            "stream": false,
            "format": schema,
            "options": { "temperature": 0.1, "num_predict": 256 }
        }))
        .send()
        .await?;

    let body: serde_json::Value = resp.json().await?;
    let content = body["message"]["content"].as_str()
        .ok_or_else(|| anyhow::anyhow!("respons Ollama kosong"))?;

    serde_json::from_str(content)
        .map_err(|e| anyhow::anyhow!("gagal parse JSON: {e}\nraw: {content}"))
}

pub async fn agent_loop(instruction: &str, max_steps: usize) -> anyhow::Result<()> {
    let mut history = format!("Instruksi user: {instruction}");

    for step in 1..=max_steps {
        let action = next_action(&history).await?;
        println!("[step {step}] -> {:?}", action);

        if matches!(action, AgentAction::Done) {
            println!("Agent selesai.");
            return Ok(());
        }

        let results = crate::execute(vec![action.clone()]);
        let result = &results[0];
        let result_str = serde_json::to_string(result)?;
        println!("  hasil: {result_str}");

        history.push_str(&format!(
            "\nLangkah {step}: menjalankan {:?}\nHasil: {result_str}",
            action
        ));

        if !result.ok {
            println!("Berhenti: ada langkah yang gagal.");
            return Ok(());
        }

        let delay = if matches!(action, AgentAction::LaunchProcess { .. }) {
            std::time::Duration::from_millis(1500)
        } else {
            std::time::Duration::from_millis(400)
        };
        tokio::time::sleep(delay).await;
    }

    println!("Berhenti: mencapai batas {max_steps} langkah.");
    Ok(())
}