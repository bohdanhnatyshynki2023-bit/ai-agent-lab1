# Skill: GitHub Actions

**Коли використовувати:** Для налаштування CI/CD пайплайну на GitHub.

**Дії:**
1. Створити файл `.github/workflows/ci.yml`.
2. Налаштувати одну job із матрицею для трьох ОС: `ubuntu-latest`, `windows-latest`, `macos-latest`[cite: 8].
3. Workflow повинен містити тільки запуск скриптів `ci.bat` (для Windows) та `ci.sh` (для Linux/macOS)[cite: 7].
4. Створити скрипти `ci.sh` та `ci.bat` у корені проєкту. Вони повинні містити команди `cargo build` та `cargo test`.

**Очікуваний результат:** Створено скрипти збірки та CI workflow. Секрети не зберігаються у файлах[cite: 8].