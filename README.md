# Rust Challenge

Сервис для генерации и анализа фейковых транзакций токенов с сохранением в
ClickHouse.

- Генерирует случайные трансферы между адресами.
- Загружает их в ClickHouse.
- Вычисляет статистику по каждому адресу:

  - общий объем
  - средние цены покупки и продажи
  - максимальный баланс по времени

> Конфигурация генерации и соединения с базой выделена в отдельный конфиг для
> удобства настройки.

---

## Архитектура

Проект разбит на отдельные модули:

- `generator` — генерация случайных трансферов.
- `storage` — работа с ClickHouse.
- `stats` — расчёт агрегатов (балансы, средние цены, объемы).
- `config` — конфигурация проекта через TOML и ENV.
- `tests` — тесты.

Логика покрыта тестами, вся работа с ошибками через `anyhow::Result` и
`.context()`.

---

## Установка (macOS)

### Установка ClickHouse

```bash
brew install clickhouse
brew services start clickhouse
```

Проверить подключение:

```bash
clickhouse-client
```

### Создание пользователя и выдача прав

Подключитесь к ClickHouse:

```bash
clickhouse-client
```

Далее выполнить:

```sql
CREATE USER rust IDENTIFIED BY 'rustpass';

GRANT CREATE, INSERT, SELECT, DROP, TRUNCATE, ALTER ON *.* TO rust;
```

---

## Конфигурация

### 1. `.env` (доступ к ClickHouse)

```env
CLICKHOUSE_URL=http://localhost:8123
CLICKHOUSE_USER=rust
CLICKHOUSE_PASSWORD=rustpass
CLICKHOUSE_DATABASE=default
```

### 2. `config.toml` (параметры генерации)

```toml
[generator]
min_amount = 5.0
max_amount = 999.9
min_price = 0.1
max_price = 2.0
max_age_secs = 604800
address_pool_amount = 300

[clickhouse]
url = ""
user = ""
password = ""
database = ""
```

> ClickHouse-секцию в `config.toml` можно оставить пустой — она читается из
> `.env`.

---

## Запуск

### Сборка

```bash
cargo build
```

### Запуск генерации и загрузки

```bash
cargo run
```

### Запуск тестов

Полный запуск:

```bash
cargo test -- --show-output
```
