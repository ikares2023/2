# Rp CVD — «Выписка рецептов для лечения ССЗ» (Tauri 2, Windows)

Обёртка над одним файлом `src/index.html`. Чтобы обновить программу, замените этот файл и пересоберите.

## Вариант A. Сборка на своём Windows

1. Установите: Node.js 20+, Rust (https://rustup.rs) и «Microsoft C++ Build Tools» (рабочая нагрузка «Разработка классических приложений на C++»). WebView2 уже есть в Windows 10/11.
2. В папке проекта:
   ```
   npm install
   npm run tauri build
   ```
3. Установщик: `src-tauri/target/release/bundle/nsis/Rp CVD_2.1.0_x64-setup.exe`

Запуск для отладки без установщика: `npm run tauri dev`.

## Вариант B. Сборка в облаке (Windows не нужен)

1. Загрузите папку в репозиторий GitHub.
2. Вкладка Actions → «Build Windows installer» → Run workflow.
3. Через 5–10 минут скачайте .exe из Artifacts.

## Настройки

- Иконка: замените `src-tauri/icons/icon-source.png` (1024×1024) и выполните `npm run tauri icon src-tauri/icons/icon-source.png`.
- Название и версия: `productName` и `version` в `src-tauri/tauri.conf.json` и `Cargo.toml`.
- Для компьютеров без интернета: в `tauri.conf.json` замените `webviewInstallMode` на `{ "type": "offlineInstaller" }` (установщик станет на ~130 МБ больше, зато WebView2 встроен).
- Данные (localStorage: сценарии, режим) хранятся в `%LOCALAPPDATA%\ru.rpcvd.prescriptions`. Если поменять `identifier`, сохранённые данные «потеряются».
