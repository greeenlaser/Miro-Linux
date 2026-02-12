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
        .title("Miro")
        .initialization_script(r#"
            (() => {
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

                //
                // MAIN CONTAINER
                //

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

                    opacity: 0;
                    pointer-events: none;
                    transition: opacity 160ms ease;
                `;

                //
                // LEFT-SIDE BACK AND FORWARD BUTTONS
                //

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

                //
                // CENTER URL BAR
                //

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

                //
                // RIGHT-SIDE HOME BUTTON
                //

                const homeBtn = mkBtn('🏠', () => {
                    window.location.href = "https://miro.com";
                });

                homeBtn.style.marginLeft = "auto";
                homeBtn.style.marginRight = "24px";

                //
                // APPEND AND DISPLAY UI
                //

                const injectUI = () => {
                    if (document.getElementById('__nav_controls')) return;

                    container.appendChild(left);
                    container.appendChild(urlBar);
                    container.appendChild(homeBtn);

                    document.documentElement.appendChild(container);

                    updateNavButtons();
                };

                injectUI();

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

                //
                // OPACITY UPDATE
                //

                let visible = false;

                const showBar = () => {
                    if (visible) return;
                    visible = true;
                    container.style.opacity = "1";
                    container.style.pointerEvents = "auto";
                };

                const hideBar = () => {
                    if (!visible) return;
                    visible = false;
                    container.style.opacity = "0";
                    container.style.pointerEvents = "none";
                };

                document.addEventListener("mousemove", (e) => {
                    const y = e.clientY;

                    if (!visible && y <= 5) {
                        showBar();
                    } else if (visible && y > 50) {
                        hideBar();
                    }
                });
            })();
        "#)
        .build()?;

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
