# RDN (Rust Day & Night)


A lightweight, high-performance system utility written in Rust that automatically switches Windows appearance themes (Dark/Light) based on the time of day. 

### Features
* **Zero Overhead**: Minimal CPU and RAM usage.
* **Smart Scheduling**: Automatically detects current time and applies the theme.
* **Safe**: Uses official Windows Registry APIs through the `winreg` crate.

### Installation
1. Ensure you have Rust installed.
2. Clone the repository and navigate to the folder.
3. Run `cargo build --release`.
4. The executable will be available in `target/release/rdn.exe`.

---

# RDN (Rust Day & Night)

Компактная и высокопроизводительная системная утилита, написанная на Rust, которая автоматически переключает темы оформления Windows (Темная/Светлая) в зависимости от времени суток.

### Основные функции
* **Нулевая нагрузка**: Минимальное потребление ресурсов процессора и оперативной памяти.
* **Умное расписание**: Автоматически определяет текущее время и применяет нужную тему.
* **Безопасность**: Использует официальные API реестра Windows через библиотеку `winreg`.

### Установка и запуск
1. Убедитесь, что у вас установлен Rust.
2. Склонируйте репозиторий и перейдите в папку проекта.
3. Выполните команду `cargo build --release`.
4. Готовый файл будет находиться по пути `target/release/rdn.exe`.это
