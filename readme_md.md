# 👋 Welcome to My Personal Website

This repository contains the source code for my personal portfolio and landing page (`adamechdira/me`). It is designed to be lightweight, fast, and clean, showcasing my work, background, and ways to connect with me.

## 🌐 Live Demo

You can visit the live website using the custom domain configured in the `CNAME` file, or via GitHub Pages:
- **Live Site:** [https://adamechdira.github.io/me](https://adamechdira.github.io/me) *(Update with your custom domain if applicable)*

---

## 📂 Project Structure

```text
me/
├── .github/
│   └── workflows/    # GitHub Actions workflows for automated deployment
├── CNAME             # Custom domain configuration for GitHub Pages
├── index.html        # Main entry point containing the layout, styles, and scripts
└── README.md         # Project documentation
```

---

## ✨ Features

- **Single-Page Architecture:** Everything is cleanly structured inside `index.html` for fast loading times and zero build overhead.
- **Responsive Design:** Optimized to look great on desktop, tablet, and mobile devices.
- **Automated Deployment:** Uses GitHub Actions (`.github/workflows`) to automatically deploy updates whenever changes are pushed to the `main` branch.
- **Custom Domain Ready:** Configured with a `CNAME` record for seamless custom domain hosting.

---

## 🚀 Getting Started (Local Development)

Since this is a static web project, you don't need any complex build tools or package managers to run it locally:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/adamechdira/me.git
   ```
2. **Navigate to the project folder:**
   ```bash
   cd me
   ```
3. **Open in your browser:**
   Simply double-click `index.html` or run a local development server (e.g., VS Code **Live Server** extension or Python):
   ```bash
   python3 -m http.server 8000
   ```
   Then visit `http://localhost:8000` in your browser.

---

## 🛠️ Deployment

Any commit pushed to the `main` branch automatically triggers the GitHub Actions workflow in `.github/workflows` to build and publish the site to GitHub Pages.

---

## 📬 Contact & Connect

- **GitHub:** [@adamechdira](https://github.com/adamechdira)
- **Website:** *(Add your custom domain link here)*
- **Email:** *(Add your email address here)*

---

© 2026 Adam Echdira. All rights reserved.