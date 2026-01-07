# Digital Fortune Cookie

A quirky web application that generates daily bizarre life advice using the Google Gemini API. Visit the site, crack open your digital fortune cookie, and receive hilariously impractical wisdom to guide your day!

## Features

- **Random Bizarre Advice** - Get absurd, hilarious life advice powered by AI
- **Web Interface** - Beautiful, interactive fortune cookie UI
- **Real-time Generation** - Advice generated on-demand using Google Gemini API
- **Easy Setup** - Simple environment configuration with `.env` file
- **Vibe Customization** - Add an optional vibe (e.g., "corporate goth energy") per request
- **Daily Streak Counter** - Track consecutive days of fortune-seeking with a visual streak display
- **Copy & Favorites** - Copy the fortune and save favorites with persistent sidebar
- **Export** - Download your history and favorites as JSON
- **Search** - Filter through History and Favorites with a single search box
- **Keyboard Shortcuts** - `c` to copy, `f` to favorite
- **Fortune Card Download** - Generate and download shareable fortune card images
- **Sound Effects** - Cookie crack sound effect when clicking

## Prerequisites

- [Google Gemini API Key](https://ai.google.dev/)

## Setup

1. **Clone or create the project:**
   ```bash
   cd digital-fortune-cookie
   ```

2. **Create a `.env` file:**
   ```bash
   cp .env.example .env
   ```

3. **Add your Gemini API Key:**
   Edit `.env` and replace `your_gemini_api_key_here` with your actual API key:
   ```
   GEMINI_API_KEY=your_actual_api_key
   PORT=8080
   ```

4. **Build and run:**
   ```bash
   cargo run
   ```

5. **Visit the site:**
   Open your browser and go to `http://127.0.0.1:8080`

## Usage

1. Click the fortune cookie 🥠 to generate (cookie-only)
2. Optionally type a "Vibe" before generating
3. Copy your fortune or add it to Favorites ⭐
4. See History and Favorites in the left sidebar; search and export anytime
5. Repeat as many times as you need guidance!

## API Endpoints

- **GET `/`** - Serves the main web interface
- **GET `/api/fortune`** - Returns a JSON object with a fortune
  ```json
  {
    "fortune": "Your bizarre advice goes here..."
  }
  ```

### Themes

- Multiple themes available: Aurora, Sunset, Midnight, Mint, Paper.
- Theme selector is in the footer.
- Options:
   - Switch themes via the dropdown
   - Randomize theme with one click
   - Auto mode to match system light/dark preference
- Your choice is saved to localStorage and persists across visits.

## Environment Variables

- `GEMINI_API_KEY` - Your Google Gemini API key (required)
- `PORT` - Server port (default: 8080)
- `PROMPT` - Customize the fortune prompt text. If not set, a default quirky prompt is used.

### Customize the prompt

Edit your `.env` to override the default prompt:

```
PROMPT="Generate one bizarre, hilariously impractical piece of life advice for today. Keep it modern and relatable."
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

## Future Ideas


## Contributing

Feel free to fork, modify, and improve this quirky project!
