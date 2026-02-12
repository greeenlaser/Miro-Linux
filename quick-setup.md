# Quick setup for a basic Tauri project

- requirements
  - sudo pacman -S webkit2gtk-4.1 gtk3 libayatana-appindicator librsvg base-devel curl file openssl rust nodejs npm
  
- check webkit
  - pkg-config --modversion webkit2gtk-4.1

- check rust and node
  - rustc --version
  - node --version
  - npm --version

- create project
  - cd ~/_webapps
  - npm create tauri-app@latest
  - project name: google-test
  - identifier: com.greenlaser.google-test
  - language: TypeScript/JavaScript
  - package manager: npm
  - ui template: vanilla
  - ui flavor: TypeScript

- add webkit to cargo
  - cd google-test/src-tauri/
  - cargo add webkit2gtk

- project setup
  - cd ..
  - npm install
  - kate src-tauri/tauri.conf.json

  - replace this
    - "beforeDevCommand": "npm run dev",

  - with this
    - "beforeDevCommand": "",
  
  - remove this line
    - "devUrl": "http://localhost:1420",
  
  - replace this
    - "beforeBuildCommand": "npm run build",

  - with this
    - "beforeBuildCommand": "",
  
  - replace this
    - "frontendDist": "../dist",
  
  -  with this
    - "frontendDist": null,
  
  - remove the entire "windows" array block

  - replace this
    - "targets": "all",

  - with this
    - "targets": [ "deb", "rpm" ],

  - disable csp blocking if it already isnt there, ensure this exists:
    - "security:" { "csp": null }

- run the app in debug mode to ensure it worked
  - WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev

---

- modify toml for release
  - kate src-tauri/Cargo.toml

- add these at the bottom
  - [profile.release]
  - strip = true
  - lto = true
  - codegen-units = 1
  - panic = "abort"

- build for release
  - npm run tauri build
