use actix_web::{web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<ContentBlock>,
}

#[derive(Debug, Serialize)]
struct ContentBlock {
    parts: Vec<TextPart>,
}

#[derive(Debug, Serialize)]
struct TextPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Serialize)]
struct FortuneResponse {
    fortune: String,
}

#[derive(Debug, Deserialize)]
struct FortuneParams {
    extra: Option<String>,
}

async fn get_fortune(query: web::Query<FortuneParams>) -> HttpResponse {
    let api_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "your_api_key_here".to_string());
    
    if api_key == "your_api_key_here" {
        return HttpResponse::InternalServerError()
            .json(FortuneResponse {
                fortune: "API key not configured. Please add GEMINI_API_KEY to your .env file.".to_string(),
            });
    }
    
    let base_prompt = env::var("PROMPT").unwrap_or_else(|_| {
        "Generate one bizarre, hilariously impractical life advice for today. Make it weird, funny, and completely absurd. Keep it to 1-2 sentences. Keep it modern and relatable.".to_string()
    });
    let extra = query.extra.clone().unwrap_or_default();
    let prompt = if extra.trim().is_empty() { base_prompt } else { format!("{}\nExtra vibe: {}", base_prompt, extra.trim()) };

    
    let client = reqwest::Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";
    
    let request_body = GeminiRequest {
        contents: vec![ContentBlock {
            parts: vec![TextPart {
                text: prompt.to_string(),
            }],
        }],
    };
    
    match client
        .post(url)
        .header("x-goog-api-key", &api_key)
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<GeminiResponse>().await {
                Ok(data) => {
                    if let Some(candidate) = data.candidates.first() {
                        if let Some(part) = candidate.content.parts.first() {
                            return HttpResponse::Ok().json(FortuneResponse {
                                fortune: part.text.clone(),
                            });
                        }
                    }
                    HttpResponse::InternalServerError()
                        .json(FortuneResponse {
                            fortune: "The cookie crumbled before revealing its wisdom...".to_string(),
                        })
                }
                Err(e) => {
                    eprintln!("Error parsing response: {:?}", e);
                    HttpResponse::InternalServerError()
                        .json(FortuneResponse {
                            fortune: "The spirits are silent today.".to_string(),
                        })
                }
            }
        }
        Err(e) => {
            eprintln!("Error calling Gemini API: {:?}", e);
            HttpResponse::InternalServerError()
                .json(FortuneResponse {
                    fortune: "Connection to the cosmic realm failed.".to_string(),
                })
        }
    }
}

async fn index() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Digital Fortune Cookie</title>
    <style>
        * { box-sizing: border-box; }
        :root {
            --bg-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            --accent: #764ba2;
            --accent-alt: #667eea;
        }
        body {
            font-family: 'Georgia', serif;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            margin: 0;
            background: var(--bg-gradient);
            transition: background 0.3s ease;
        }
        /* Themes */
        .theme-aurora { --bg-gradient: linear-gradient(135deg, #667eea 0%, #764ba2 100%); --accent:#764ba2; --accent-alt:#667eea; }
        .theme-sunset { --bg-gradient: linear-gradient(135deg, #ff7e5f 0%, #feb47b 100%); --accent:#ff7e5f; --accent-alt:#feb47b; }
        .theme-midnight { --bg-gradient: linear-gradient(135deg, #232526 0%, #414345 100%); --accent:#91a7ff; --accent-alt:#748ffc; }
        .theme-mint { --bg-gradient: linear-gradient(135deg, #a8ff78 0%, #78ffd6 100%); --accent:#2b8a3e; --accent-alt:#20c997; }
        .theme-paper { --bg-gradient: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%); --accent:#6c5ce7; --accent-alt:#a29bfe; }
        header {
            background-color: rgba(0, 0, 0, 0.2);
            color: white;
            padding: 20px;
            text-align: center;
            border-bottom: 2px solid rgba(255, 255, 255, 0.1);
        }
        header h1 {
            margin: 0;
            font-size: 2em;
        }
        .streak-counter {
            display: inline-block;
            background-color: rgba(255, 255, 255, 0.2);
            padding: 8px 16px;
            border-radius: 20px;
            margin-top: 10px;
            font-size: 1em;
            backdrop-filter: blur(10px);
        }
        .streak-counter span {
            font-weight: bold;
            color: #ffd700;
            font-size: 1.2em;
        }
        .theme-select {
            padding: 8px 12px;
            border-radius: 8px;
            border: none;
        }
        .content-wrapper {
            display: flex;
            flex: 1;
            overflow-y: auto;
        }
        main {
            flex: 1;
            display: flex;
            justify-content: center;
            align-items: flex-start;
            padding: 40px 20px;
        }
        .container {
            text-align: center;
            background: white;
            padding: 40px;
            border-radius: 20px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
            max-width: 500px;
        }
        h1 {
            color: var(--accent);
            margin: 0 0 30px 0;
            font-size: 2.5em;
        }
        .cookie {
            font-size: 80px;
            margin: 20px 0;
            cursor: pointer;
            transition: transform 0.3s ease;
        }
        .cookie:hover {
            transform: scale(1.1) rotate(10deg);
        }
        @keyframes crack {
            0% { transform: scale(1) rotate(0deg); }
            30% { transform: scale(1.15) rotate(-8deg); }
            60% { transform: scale(0.95) rotate(8deg); }
            100% { transform: scale(1) rotate(0deg); }
        }
        .cookie.crack {
            animation: crack 0.6s ease;
        }
        .controls { display: flex; gap: 8px; justify-content: center; flex-wrap: wrap; }
        .vibe-input {
            width: 100%;
            max-width: 400px;
            padding: 10px 12px;
            border-radius: 10px;
            border: 1px solid #ddd;
            margin: 8px auto 0;
        }
        button {
            background-color: var(--accent);
            color: white;
            border: none;
            padding: 12px 30px;
            font-size: 16px;
            border-radius: 25px;
            cursor: pointer;
            transition: background-color 0.3s ease;
            margin: 5px;
        }
        button:hover {
            background-color: var(--accent-alt);
        }
        button:disabled {
            background-color: #ccc;
            cursor: not-allowed;
        }
        .fortune {
            margin-top: 30px;
            font-size: 18px;
            font-style: italic;
            color: #333;
            background: #f5f5f5;
            padding: 20px;
            border-radius: 10px;
            line-height: 1.6;
            text-align: left;
            word-wrap: break-word;
        }
        .loading {
            color: var(--accent);
            font-size: 14px;
        }
        .sidebar {
            position: fixed;
            left: 0;
            top: 60px;
            bottom: 0;
            width: 280px;
            background-color: rgba(255, 255, 255, 0.95);
            padding: 20px;
            overflow-y: auto;
            border-right: 2px solid rgba(255, 255, 255, 0.1);
            display: flex;
            flex-direction: column;
        }
        .search-input {
            width: 100%;
            padding: 8px 10px;
            border-radius: 8px;
            border: 1px solid #ddd;
            margin: 8px 0 12px;
        }
        .sidebar h2 {
            color: var(--accent);
            margin-top: 0;
            font-size: 1.1em;
        }
        .history-list {
            list-style: none;
            padding: 0;
            margin: 0;
            flex: 1;
            overflow-y: auto;
        }
        .history-item {
            background: #f5f5f5;
            padding: 10px;
            margin: 8px 0;
            border-radius: 8px;
            font-size: 12px;
            line-height: 1.3;
            color: #333;
            border-left: 3px solid var(--accent);
            cursor: pointer;
            transition: all 0.3s ease;
            word-wrap: break-word;
            overflow-wrap: break-word;
            word-break: break-word;
            max-height: 80px;
            overflow: hidden;
        }
        .history-item:hover {
            background: #e8e8e8;
            transform: translateX(-5px);
        }
        .clear-btn {
            width: 100%;
            padding: 10px;
            background-color: #d32f2f;
            font-size: 13px;
            margin-top: auto;
        }
        .clear-btn:hover {
            background-color: #b71c1c;
        }
        .empty-message {
            color: #999;
            text-align: center;
            padding: 20px;
            font-size: 13px;
        }
        footer {
            background-color: rgba(0, 0, 0, 0.2);
            color: white;
            padding: 20px;
            text-align: center;
            border-top: 2px solid rgba(255, 255, 255, 0.1);
            font-size: 14px;
            margin-top: auto;
        }
        footer a {
            color: #fff;
            text-decoration: none;
            transition: opacity 0.3s ease;
        }
        footer a:hover {
            opacity: 0.8;
        }
        .footer-controls {
            margin-top: 10px;
            display: flex;
            gap: 10px;
            align-items: center;
            justify-content: center;
            flex-wrap: wrap;
        }
        .footer-controls label {
            font-weight: 600;
        }
        @media (max-width: 768px) {
            .sidebar {
                display: none;
            }
        }
    </style>
</head>
<body>
    <header>
        <h1>🥠 Digital Fortune Cookie</h1>
        <div class="streak-counter">🔥 Daily Streak: <span id="streak-count">0</span> days</div>
    </header>

    <div class="content-wrapper">
        <main>
            <div class="container">
                <p>Click the cookie to receive your daily bizarre life advice!</p>
                <input type="text" id="vibeInput" class="vibe-input" placeholder="Add a vibe (optional, e.g., 'corporate goth energy')" maxlength="120">
                <div class="cookie" id="cookie">🥠</div>
                <div class="controls">
                    <button id="copyBtn">Copy</button>
                    <button id="favBtn">⭐ Favorite</button>
                    <button id="downloadCardBtn">Download Card</button>
                </div>
                <div class="fortune" id="fortune">Click the button to reveal your fortune...</div>
            </div>
        </main>

        <aside class="sidebar">
            <h2>📜 Fortune History</h2>
            <input id="searchInput" class="search-input" placeholder="Search history & favorites..." />
            <ul class="history-list" id="historyList">
                <div class="empty-message">No fortunes yet...</div>
            </ul>
            <button class="clear-btn" onclick="clearHistory()">Clear History</button>
            <h2>⭐ Favorites</h2>
            <ul class="history-list" id="favoritesList">
                <div class="empty-message">No favorites yet...</div>
            </ul>
            <button class="clear-btn" onclick="clearFavorites()">Clear Favorites</button>
            <button class="clear-btn" onclick="exportData()">Export JSON</button>
        </aside>
    </div>

    <footer>
        <p>Made with 🥠 | <a href="https://github.com/SeradedStripes/digital-fortune-cookie" target="_blank">View on GitHub</a></p>
        <div class="footer-controls">
            <label for="themeSelect">Theme:</label>
            <select id="themeSelect" class="theme-select">
                <option value="aurora">Aurora</option>
                <option value="sunset">Sunset</option>
                <option value="midnight">Midnight</option>
                <option value="mint">Mint</option>
                <option value="paper">Paper</option>
            </select>
            <button id="randomThemeBtn">Randomize</button>
            <label>
                <input type="checkbox" id="autoTheme"> Auto (match system)
            </label>
        </div>
    </footer>

    <script>
        const HISTORY_KEY = 'fortuneCookieHistory';
        const FAVORITES_KEY = 'fortuneCookieFavorites';
        const THEME_KEY = 'fortuneCookieTheme';
        const THEME_AUTO_KEY = 'fortuneCookieThemeAuto';
        const MAX_HISTORY = 20;
        const THEMES = ['aurora','sunset','midnight','mint','paper'];

        // Simple cookie crack sound effect using Web Audio API
        function playCrackSound() {
            const ctx = new (window.AudioContext || window.webkitAudioContext)();
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();
            osc.connect(gain);
            gain.connect(ctx.destination);
            osc.frequency.value = 180;
            gain.gain.setValueAtTime(0.3, ctx.currentTime);
            gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.1);
            osc.start(ctx.currentTime);
            osc.stop(ctx.currentTime + 0.1);
        }

        // Streak Management
        function loadStreak() {
            const streakData = localStorage.getItem('fortuneStreak');
            if (streakData) {
                return JSON.parse(streakData);
            }
            return { lastDate: null, count: 0 };
        }

        function saveStreak(streak) {
            localStorage.setItem('fortuneStreak', JSON.stringify(streak));
        }

        function getTodayDate() {
            const now = new Date();
            return now.toISOString().split('T')[0]; // YYYY-MM-DD
        }

        function getYesterdayDate() {
            const yesterday = new Date();
            yesterday.setDate(yesterday.getDate() - 1);
            return yesterday.toISOString().split('T')[0];
        }

        function updateStreak() {
            const streak = loadStreak();
            const today = getTodayDate();
            
            if (streak.lastDate === today) {
                // Already got a fortune today, no change
                return streak.count;
            } else if (streak.lastDate === getYesterdayDate()) {
                // Continuing streak from yesterday
                streak.count += 1;
            } else if (streak.lastDate === null) {
                // First time user
                streak.count = 1;
            } else {
                // Missed a day, reset streak
                streak.count = 1;
            }
            
            streak.lastDate = today;
            saveStreak(streak);
            return streak.count;
        }

        function displayStreak() {
            const streak = loadStreak();
            const today = getTodayDate();
            
            // If last date is not today or yesterday, reset display to 0
            if (streak.lastDate !== today && streak.lastDate !== getYesterdayDate() && streak.lastDate !== null) {
                document.getElementById('streak-count').textContent = '0';
            } else {
                document.getElementById('streak-count').textContent = streak.count;
            }
        }

        function applyTheme(theme) {
            const body = document.body;
            body.classList.remove('theme-aurora','theme-sunset','theme-midnight','theme-mint','theme-paper');
            const cls = `theme-${theme}`;
            body.classList.add(cls);
            localStorage.setItem(THEME_KEY, theme);
        }

        function systemTheme() {
            const dark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
            return dark ? 'midnight' : 'aurora';
        }

        function initTheme() {
            const select = document.getElementById('themeSelect');
            const randomBtn = document.getElementById('randomThemeBtn');
            const autoChk = document.getElementById('autoTheme');

            const autoSaved = localStorage.getItem(THEME_AUTO_KEY) === 'true';
            autoChk.checked = autoSaved;

            let initial = localStorage.getItem(THEME_KEY) || 'aurora';
            if (autoSaved) {
                initial = systemTheme();
            }
            select.value = initial;
            applyTheme(initial);

            select.addEventListener('change', (e) => {
                autoChk.checked = false;
                localStorage.setItem(THEME_AUTO_KEY, 'false');
                applyTheme(e.target.value);
            });

            randomBtn.addEventListener('click', () => {
                autoChk.checked = false;
                localStorage.setItem(THEME_AUTO_KEY, 'false');
                const next = THEMES[Math.floor(Math.random()*THEMES.length)];
                select.value = next;
                applyTheme(next);
            });

            autoChk.addEventListener('change', () => {
                const enabled = autoChk.checked;
                localStorage.setItem(THEME_AUTO_KEY, enabled ? 'true' : 'false');
                if (enabled) {
                    const t = systemTheme();
                    select.value = t;
                    applyTheme(t);
                }
            });

            if (window.matchMedia) {
                const mq = window.matchMedia('(prefers-color-scheme: dark)');
                mq.addEventListener('change', () => {
                    if (autoChk.checked) {
                        const t = systemTheme();
                        select.value = t;
                        applyTheme(t);
                    }
                });
            }
        }

        function getFortunes() {
            const data = localStorage.getItem(HISTORY_KEY);
            return data ? JSON.parse(data) : [];
        }

        function saveFortune(fortune) {
            let fortunes = getFortunes();
            const timestamp = new Date().toLocaleString();
            fortunes.unshift({ text: fortune, time: timestamp });
            fortunes = fortunes.slice(0, MAX_HISTORY);
            localStorage.setItem(HISTORY_KEY, JSON.stringify(fortunes));
            updateHistoryDisplay();
        }

        function updateHistoryDisplay(filterText = '') {
            const historyList = document.getElementById('historyList');
            const fortunes = getFortunes();
            
            if (fortunes.length === 0) {
                historyList.innerHTML = '<div class="empty-message">No fortunes yet...</div>';
                return;
            }
            
            const q = filterText.toLowerCase();
            const filtered = fortunes.filter(f => !q || f.text.toLowerCase().includes(q) || (f.time && f.time.toLowerCase().includes(q)));
            if (filtered.length === 0) {
                historyList.innerHTML = '<div class="empty-message">No matches...</div>';
                return;
            }
            historyList.innerHTML = filtered.map((f, idx) => `
                <li class="history-item" onclick="showFortune('${escapeHtml(f.text)}')">
                    <strong>#${idx + 1}</strong><br>
                    ${escapeHtml(f.text.substring(0, 100))}${f.text.length > 100 ? '...' : ''}<br>
                    <small>${f.time}</small>
                </li>
            `).join('');
        }

        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }

        function showFortune(fortune) {
            document.getElementById('fortune').textContent = fortune;
            document.getElementById('fortune').classList.remove('loading');
        }

        function clearHistory() {
            if (confirm('Are you sure you want to clear all fortune history?')) {
                localStorage.removeItem(HISTORY_KEY);
                updateHistoryDisplay();
            }
        }

        async function getFortune() {
            const fortune = document.getElementById('fortune');
            const vibe = document.getElementById('vibeInput').value || '';
            
            fortune.textContent = 'Loading your fortune...';
            fortune.classList.add('loading');
            
            try {
                const response = await fetch('/api/fortune?extra=' + encodeURIComponent(vibe));
                const data = await response.json();
                fortune.textContent = data.fortune;
                fortune.classList.remove('loading');
                saveFortune(data.fortune);
                
                // Update streak
                const newStreak = updateStreak();
                document.getElementById('streak-count').textContent = newStreak;
            } catch (error) {
                fortune.textContent = 'The cookie is too shy to speak...';
                fortune.classList.remove('loading');
            }
        }
                // Favorites handling
                function getFavorites() {
                    const data = localStorage.getItem(FAVORITES_KEY);
                    return data ? JSON.parse(data) : [];
                }

                function saveFavorite(text) {
                    const favs = getFavorites();
                    if (!favs.includes(text)) {
                        favs.unshift(text);
                        localStorage.setItem(FAVORITES_KEY, JSON.stringify(favs.slice(0, MAX_HISTORY)));
                    }
                    updateFavoritesDisplay();
                }

                function removeFavorite(text) {
                    let favs = getFavorites().filter(t => t !== text);
                    localStorage.setItem(FAVORITES_KEY, JSON.stringify(favs));
                    updateFavoritesDisplay();
                }

                function toggleFavoriteCurrent() {
                    const text = document.getElementById('fortune').textContent.trim();
                    if (!text || text.includes('Click the button')) return;
                    const favs = getFavorites();
                    if (favs.includes(text)) {
                        removeFavorite(text);
                    } else {
                        saveFavorite(text);
                    }
                }

                function updateFavoritesDisplay(filterText = '') {
                    const list = document.getElementById('favoritesList');
                    const favs = getFavorites();
                    if (favs.length === 0) { list.innerHTML = '<div class="empty-message">No favorites yet...</div>'; return; }
                    const q = filterText.toLowerCase();
                    const filtered = favs.filter(t => !q || t.toLowerCase().includes(q));
                    if (filtered.length === 0) { list.innerHTML = '<div class="empty-message">No matches...</div>'; return; }
                    list.innerHTML = filtered.map((t, idx) => `
                        <li class="history-item">
                            <strong>★ #${idx + 1}</strong><br>
                            ${escapeHtml(t.substring(0, 100))}${t.length > 100 ? '...' : ''}
                            <div>
                                <button style="margin-top:6px;padding:6px 10px;border-radius:10px" onclick="document.getElementById('fortune').textContent='${escapeHtml(t)}'">Show</button>
                                <button style="margin-top:6px;padding:6px 10px;border-radius:10px;background:#b71c1c" onclick="removeFavorite('${escapeHtml(t)}')">Remove</button>
                            </div>
                        </li>
                    `).join('');
                }

                function clearFavorites() {
                    if (confirm('Clear all favorites?')) {
                        localStorage.removeItem(FAVORITES_KEY);
                        updateFavoritesDisplay();
                    }
                }

                // Copy fortune
                async function copyFortune() {
                    const text = document.getElementById('fortune').textContent.trim();
                    if (!text) return;
                    try {
                        await navigator.clipboard.writeText(text);
                        alert('Fortune copied!');
                    } catch {
                        // Fallback
                        const ta = document.createElement('textarea');
                        ta.value = text; document.body.appendChild(ta); ta.select(); document.execCommand('copy'); document.body.removeChild(ta);
                        alert('Fortune copied!');
                    }
                }

                // Share removed: generation is cookie-only, sharing disabled.

                // Export
                function exportData() {
                    const data = {
                        history: getFortunes(),
                        favorites: getFavorites()
                    };
                    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
                    const url = URL.createObjectURL(blob);
                    const a = document.createElement('a');
                    a.href = url; a.download = 'digital-fortune-cookie-export.json';
                    document.body.appendChild(a); a.click();
                    setTimeout(() => { document.body.removeChild(a); URL.revokeObjectURL(url); }, 0);
                }
        
        // Download fortune card as image
        function downloadFortuneCard() {
            const text = document.getElementById('fortune').textContent.trim();
            if (!text || text.includes('Click the button')) return;
            
            const canvas = document.createElement('canvas');
            canvas.width = 800;
            canvas.height = 600;
            const ctx = canvas.getContext('2d');
            
            // Background gradient
            const gradient = ctx.createLinearGradient(0, 0, 800, 600);
            gradient.addColorStop(0, '#667eea');
            gradient.addColorStop(1, '#764ba2');
            ctx.fillStyle = gradient;
            ctx.fillRect(0, 0, 800, 600);
            
            // Fortune box
            ctx.fillStyle = 'rgba(255, 255, 255, 0.95)';
            ctx.roundRect(50, 100, 700, 400, 20);
            ctx.fill();
            
            // Fortune emoji
            ctx.font = '80px serif';
            ctx.fillText('🥠', 360, 80);
            
            // Fortune text
            ctx.fillStyle = '#333';
            ctx.font = 'italic 28px Georgia, serif';
            ctx.textAlign = 'center';
            const words = text.split(' ');
            let line = '';
            let y = 200;
            const maxWidth = 640;
            for (let word of words) {
                const testLine = line + word + ' ';
                const metrics = ctx.measureText(testLine);
                if (metrics.width > maxWidth && line !== '') {
                    ctx.fillText(line, 400, y);
                    line = word + ' ';
                    y += 40;
                } else {
                    line = testLine;
                }
            }
            ctx.fillText(line, 400, y);
            
            // Timestamp
            ctx.font = '16px sans-serif';
            ctx.fillStyle = '#666';
            ctx.fillText(new Date().toLocaleString(), 400, 520);
            
            // Download
            canvas.toBlob((blob) => {
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'fortune-cookie-card.png';
                document.body.appendChild(a);
                a.click();
                setTimeout(() => { document.body.removeChild(a); URL.revokeObjectURL(url); }, 0);
            });
        }

        // Cookie-only trigger for generation
        document.getElementById('cookie').addEventListener('click', () => {
            playCrackSound();
            const el = document.getElementById('cookie');
            el.classList.add('crack');
            setTimeout(() => el.classList.remove('crack'), 600);
            getFortune();
        });
        document.getElementById('copyBtn').addEventListener('click', copyFortune);
        document.getElementById('favBtn').addEventListener('click', toggleFavoriteCurrent);
        document.getElementById('downloadCardBtn').addEventListener('click', downloadFortuneCard);
        document.getElementById('searchInput').addEventListener('input', (e) => {
            const q = e.target.value || '';
            updateHistoryDisplay(q);
            updateFavoritesDisplay(q);
        });
        
        // Load on page start
        updateHistoryDisplay();
        updateFavoritesDisplay();
        initTheme();
        displayStreak();

        // Keyboard shortcuts: c (copy), f (favorite)
        document.addEventListener('keydown', (e) => {
            const active = document.activeElement;
            const typing = active && (active.tagName === 'INPUT' || active.tagName === 'TEXTAREA');
            if (typing) return;
            if (e.key.toLowerCase() === 'c') { copyFortune(); }
            else if (e.key.toLowerCase() === 'f') { toggleFavoriteCurrent(); }
        });
    </script>
</body>
</html>
    "#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    println!("🥠 Digital Fortune Cookie Server running on http://127.0.0.1:{}/demos/digital-fortune-cookie/", port);
    
    HttpServer::new(|| {
        App::new()
            .route("/demos/digital-fortune-cookie/", web::get().to(index))
            .route("/demos/digital-fortune-cookie/api/fortune", web::get().to(get_fortune))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
