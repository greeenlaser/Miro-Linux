// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewWindowBuilder, WebviewUrl};

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        WebviewWindowBuilder::new(
            app,
            "main",
            WebviewUrl::External("https://miro.com".parse().unwrap())
        )
        .initialization_script(r#"
        (() => {
        //skip running inside iframes

        if (window.top !== window.self) return;

        //new page opens in current page

        window.open = function (url) {
        if (url) {
            window.location.assign(url);
    }
    return window;
    };

    document.addEventListener('click', function (e) {
    const target = e.target instanceof Element ? e.target : null;
    const link = target?.closest('a[target="_blank"]');
        if (link && link.href) {
            e.preventDefault();
        window.location.href = link.href;
    }
    });

        const container = document.createElement('div');
        container.id = '__nav_controls';

        //main container

        container.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 42px;
        z-index: 999999;
        display: flex;
        align-items: center;
        padding: 0 12px;

        background: rgba(20,20,20,0.95);
        backdrop-filter: blur(6px);
        `;

        //left-side back and forward buttons

        const mkBtn = (label, action) => {
        const b = document.createElement('button');
        b.textContent = label;
        b.style.cssText = `
        width: 30px;
        height: 30px;
        border-radius: 6px;
        border: none;
        background: rgba(40,40,40,0.9);
        color: white;
        cursor: pointer;
        font-size: 14px;
        `;
        b.onclick = action;
        return b;
    };

    const left = document.createElement('div');
        left.style.display = 'flex';
        left.style.gap = '6px';

        const backBtn = mkBtn('<', () => history.back());
        const forwardBtn = mkBtn('>', () => history.forward());

        left.appendChild(backBtn);
        left.appendChild(forwardBtn);

        //center read-only url bar

        const urlBar = document.createElement('input');
        urlBar.type = 'text';
        urlBar.value = location.href;

        urlBar.style.cssText = `
        position: absolute;
        left: 50%;
        transform: translateX(-50%);
        width: 50%;
        max-width: 600px;
        height: 28px;
        border-radius: 6px;
        border: none;
        padding: 0 18px;
        background: rgba(60,60,60,0.9);
        color: white;
        text-align: center;
        font-size: 13px;
        `;

        const updateUrl = () => {
        urlBar.value = location.href;
    };

    urlBar.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
        let value = urlBar.value.trim();

        if (!value.startsWith('http://') && !value.startsWith('https://')) {
            value = 'https://' + value;
    }

    window.location.href = value;
    }
    })

        const updateNavButtons = () => {
        backBtn.disabled = history.length <= 1;
        backBtn.style.opacity = backBtn.disabled ? "0.4" : "1";

        forwardBtn.disabled = false;
        forwardBtn.style.opacity = "1";
    };

    window.addEventListener('popstate', () => {
    updateUrl();
        updateNavButtons();
    });

        window.addEventListener('hashchange', () => {
        updateUrl();
        updateNavButtons();
    });

        const push = history.pushState;
        history.pushState = function () {
        push.apply(this, arguments);
        updateUrl();
        updateNavButtons();
    };

    const replace = history.replaceState;
    history.replaceState = function () {
    replace.apply(this, arguments);
        updateUrl();
        updateNavButtons();
    };

    //right-side home button

    const homeBtn = mkBtn('🏠', () => {
    window.location.href = "https://miro.com";
    });

        homeBtn.style.marginLeft = "auto";
        homeBtn.style.marginRight = "24px";

        //final setup

        const injectUI = () => {
        if (document.getElementById('__nav_controls')) return;

        container.appendChild(left);
        container.appendChild(urlBar);
        container.appendChild(homeBtn);

        document.body.prepend(container);

        const style = document.createElement("style");
        style.textContent = `
        body > *:not(#__nav_controls) {
        margin-top: 42px !important;
    }
    `;
    document.head.appendChild(style);

        updateNavButtons();
    };

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", injectUI);
    } else {
        injectUI();
    }

    //ensure UI stays alive even with full document rebuild

    const ensureChromeAlive = () => {
    if (!document.getElementById('__nav_controls')) {
        injectUI();
    }
    };

    const observer = new MutationObserver(() => {
    ensureChromeAlive();
    });

        observer.observe(document.documentElement, {
        childList: true,
        subtree: true
    });
    })();
        "#)
        .build()?;

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
