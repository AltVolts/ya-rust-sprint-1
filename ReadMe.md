# Проектная работа модуля 1. Чтение, парсинг и анализ данных в Rust

## Проект содержит cargo workspace, состоящий из трёх основных крейтов:
1. Библиотечный крейт `parser` - ядро парсинга записей банковских транзакций 
   и их сериализация в разные форматы данных;
2. Бинарный крейт `cli-comparer` - Консольное приложение, использующее функциональность парсеров из lib-крейта;
3. Бинарный крейт `cli-converter` - консольное приложение, использующее функциональность парсеров из lib-крейта.

## Структура проекта:
```text
ya-rust-sprint-1
├── Cargo.lock
├── Cargo.toml
├── cli-comparer/
├── cli-converter/
├── parser/
├── README.md
├── test_files/
└── Спецификация_форматов/
```

Где:
1. ```cli-comparer/``` - директория с кодом бинарного крейта `cli-comparer`
2. ```cli-converter/``` - директория с кодом бинарного крейта `cli-converter`
3. ```parser/``` - директория с кодом библиотечного крейта `parser`
4. ```test_files/``` - директория с файлами-примерами форматов данных
5. ```Спецификация_форматов/``` - директория со спецификацией форматов данных

## Демонстрация использования:
1. `cli-comparer`:
```shell
cargo comparer --file1 ./test_files/records_example.bin --format1 binary --file2 ./test_files/records_example.txt --format2 txt
```
2. `cli-converter`: 
```shell 
cargo converter --input ./test_files/records_example.csv --input-format csv --output-format txt > output_file.txt
```

