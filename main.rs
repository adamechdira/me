use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Profile {
    name: &'static str,
    role: &'static str,
    location: &'static str,
    github: &'static str,
    projects: Vec<&'static str>,
    stack: Vec<&'static str>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(portfolio_page))
        .route("/api/profile", get(api_profile));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 السيرفر خدام على: http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn api_profile() -> Json<Profile> {
    Json(Profile {
        name: "Adam Echdira",
        role: "Software Developer & Researcher in Sharia and Law",
        location: "Taroudant, Morocco",
        github: "https://github.com/adamechdira",
        projects: vec!["IronCraftOS", "Local LLM Pipelines (Ollama)", "Agrinisa"],
        stack: vec!["Rust", "Python", "Linux/KVM", "Azure", "Angular", "Tailscale"],
    })
}

async fn portfolio_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Adam Echdira | آدم أشديرة</title>
    <style>
        :root {
            --bg: #0b0f19;
            --card: #131a2a;
            --accent: #38bdf8;
            --accent-secondary: #f97316;
            --text: #f1f5f9;
            --muted: #94a3b8;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: system-ui, -apple-system, 'Segoe UI', Roboto, sans-serif;
            background-color: var(--bg);
            color: var(--text);
            line-height: 1.7;
            padding: 2rem 1rem;
        }
        .container { max-width: 900px; margin: 0 auto; }
        header {
            text-align: center;
            padding: 3rem 1.5rem;
            background: linear-gradient(135deg, #131a2a 0%, #1e293b 100%);
            border-radius: 16px;
            border: 1px solid #1e293b;
            margin-bottom: 2rem;
        }
        .badge {
            display: inline-block;
            background: rgba(249, 115, 22, 0.15);
            color: var(--accent-secondary);
            padding: 0.3rem 0.9rem;
            border-radius: 999px;
            font-size: 0.85rem;
            font-weight: 600;
            margin-bottom: 1rem;
            border: 1px solid rgba(249, 115, 22, 0.3);
        }
        h1 { font-size: 2.5rem; margin-bottom: 0.5rem; color: #fff; }
        .subtitle { color: var(--accent); font-size: 1.15rem; font-weight: 500; }
        .location { color: var(--muted); font-size: 0.95rem; margin-top: 0.5rem; }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }
        .card {
            background: var(--card);
            padding: 1.75rem;
            border-radius: 14px;
            border: 1px solid #1e293b;
            transition: transform 0.2s ease, border-color 0.2s ease;
        }
        .card:hover { transform: translateY(-4px); border-color: var(--accent); }
        .card h2 { color: var(--accent); font-size: 1.25rem; margin-bottom: 1rem; }
        ul { list-style: none; }
        li { margin-bottom: 0.65rem; color: #cbd5e1; position: relative; padding-right: 1.2rem; }
        li::before { content: "▹"; position: absolute; right: 0; color: var(--accent-secondary); }
        .tags { display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 1rem; }
        .tag {
            background: #0f172a;
            color: var(--accent);
            padding: 0.25rem 0.7rem;
            border-radius: 6px;
            font-size: 0.85rem;
            border: 1px solid #1e293b;
            direction: ltr;
        }
        .links { display: flex; justify-content: center; gap: 1rem; flex-wrap: wrap; margin-top: 1.5rem; }
        .btn {
            text-decoration: none;
            padding: 0.65rem 1.4rem;
            border-radius: 8px;
            font-weight: 600;
            transition: opacity 0.2s;
            direction: ltr;
        }
        .btn-primary { background: var(--accent); color: #0f172a; }
        .btn-outline { border: 1px solid var(--accent); color: var(--accent); }
        .btn:hover { opacity: 0.85; }
        footer {
            text-align: center;
            color: var(--muted);
            font-size: 0.9rem;
            margin-top: 3rem;
            padding-top: 1.5rem;
            border-top: 1px solid #1e293b;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <span class="badge">🦀 Built with Rust & Axum</span>
            <h1>آدم أشديرة | Adam Echdira</h1>
            <p class="subtitle">مطور برمجيات على GitHub & باحث في الشريعة والقانون</p>
            <p class="location">📍 تارودانت، جهة سوس ماسة، المغرب</p>
            <div class="links">
                <a href="https://github.com/adamechdira" target="_blank" class="btn btn-primary">GitHub Profile</a>
                <a href="https://ironcraftos.vercel.app" target="_blank" class="btn btn-outline">IronCraftOS</a>
                <a href="/api/profile" target="_blank" class="btn btn-outline">JSON API</a>
            </div>
        </header>

        <div class="grid">
            <div class="card">
                <h2>💻 التطوير والبرمجة المفتوحة</h2>
                <ul>
                    <li>تطوير ونشر المشاريع البرمجية على منصة <strong>GitHub</strong>.</li>
                    <li>بناء تطبيقات ويب وتجارب أنظمة تفاعلية مثل <strong>IronCraftOS</strong>.</li>
                    <li>برمجة أدوات الأتمتة والأنظمة الخلفية باستعمال <strong>Rust</strong> و <strong>Python</strong>.</li>
                </ul>
            </div>

            <div class="card">
                <h2>🤖 الذكاء الاصطناعي والأنظمة</h2>
                <ul>
                    <li>تشغيل وضبط نماذج اللغة المحلية عبر <strong>Ollama</strong> و Unsloth.</li>
                    <li>إدارة خوادم <strong>Ubuntu VPS</strong> والأنظمة الافتراضية <strong>KVM</strong> على Azure.</li>
                    <li>ربط الشبكات الآمنة باستعمال <strong>Tailscale</strong> و Cloudflare.</li>
                </ul>
            </div>

            <div class="card">
                <h2>⚖️ الخلفية الأكاديمية</h2>
                <ul>
                    <li><strong>الإجازة في الشريعة والقانون</strong> (بميزة مشرف - 15.61).</li>
                    <li>إلمام معمق بالفقه المالكي والقواعد الفقهية.</li>
                    <li>الجمع بين الصرامة القانونية والهندسة البرمجية الحديثة.</li>
                </ul>
            </div>

            <div class="card">
                <h2>🛠️ التقنيات والأدوات</h2>
                <div class="tags">
                    <span class="tag">Rust</span>
                    <span class="tag">Axum</span>
                    <span class="tag">Git / GitHub</span>
                    <span class="tag">Python</span>
                    <span class="tag">Linux / KVM</span>
                    <span class="tag">Ollama</span>
                    <span class="tag">Azure</span>
                    <span class="tag">Angular</span>
                </div>
            </div>
        </div>

        <footer>
            <p>تم التطوير بلغة Rust ⚡ | © 2026 Adam Echdira (idxv.me)</p>
        </footer>
    </div>
</body>
</html>"#,
    )
}
