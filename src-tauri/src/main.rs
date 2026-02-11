// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};
use std::sync::OnceLock;

fn main() {
    static INJECTED: OnceLock<()> = OnceLock::new();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_for_eval = window.clone();

            window.on_window_event(move |event| {
                if matches!(event, WindowEvent::Resized(_)) {
                    // Inject exactly once
                    if INJECTED.set(()).is_ok() {
                            let _ = window_for_eval.eval(r#"
                            (() => {
                                if (document.getElementById('__nav_controls')) return;

                                const container = document.createElement('div');
                                container.id = '__nav_controls';

                                container.style.cssText = `
                                    position: fixed;
                                    top: 0;
                                    left: 8px;
                                    z-index: 999999;
                                    display: flex;
                                    gap: 6px;

                                    transform: translateY(-110%);
                                    opacity: 0;
                                    transition:
                                    transform 220ms cubic-bezier(.2,.8,.2,1), opacity 180ms ease;

                                    pointer-events: none; `;

                                const mkBtn = (label, action) => {
                                    const b = document.createElement('button');
                                    b.textContent = label;
                                    b.style.cssText = `
                                        width: 28px;
                                        height: 28px;
                                        border-radius: 6px;
                                        border: none;
                                        background: rgba(30,30,30,0.85);
                                        color: white;
                                        cursor: pointer; `;
                                    b.onclick = action;
                                    return b;
                                };

                                container.appendChild(mkBtn('←', () => history.back()));
                                container.appendChild(mkBtn('→', () => history.forward()));
                                document.body.appendChild(container);

                                let visible = false;
                                let hoveringControls = false;

                                const show = () => {
                                    if (visible) return;
                                    visible = true;
                                    container.style.transform = 'translateY(8px)';
                                    container.style.opacity = '1';
                                    container.style.pointerEvents = 'auto';
                                };

                                const hide = () => {
                                    if (!visible || hoveringControls) return;
                                    visible = false;
                                    container.style.transform = 'translateY(-110%)';
                                    container.style.opacity = '0';
                                    container.style.pointerEvents = 'none';
                                };

                                container.addEventListener('mouseenter', () => {
                                    hoveringControls = true;
                                });

                                container.addEventListener('mouseleave', () => {
                                    hoveringControls = false;
                                });

                                document.addEventListener('mousemove', (e) => {
                                    // --- hysteresis zones ---
                                    const showZone = e.clientX < 90 && e.clientY < 50;
                                    const hideZone = e.clientX > 130 || e.clientY > 80;

                                    if (showZone) show();
                                    else if (hideZone) hide();
                                });
                        })();
                    "#);
                }
            }
        });

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
