# **Крейт `cli-comparer`** (консольное приложение)

## Программа для сравнения двух файлов с записями банковских транзакций

Консольная утилита `cli-comparer`использует функционал библиотеки `parser`.
Данная программа позволяет загружать транзакции из двух файлов и сравнивать их. 
В случае обнаружения различий программа сообщает, какие именно транзакции отличаются.

## Формат

### Структура утилиты
`cli-comparer --file1 <path1> --format1 <format> --file2 <path2> --format2 <format>`

### Аргументы
`--file1 <path>` — путь к первому файлу.

`--format1 <format>` — формат первого файла (`binary`, `csv`, `txt`).

`--file2 <path>` — путь ко второму файлу.

`--format2 <format> `— формат второго файла (`binary`, `csv`, `txt`).

### Выходные данные
Если записи в файлах полностью совпадают, утилита выводит сообщение об успехе:
`The transaction records are identical.`

При несовпадении выводится информация о различающихся записях:
```shell
Transaction 1000000000000000 differs:
  In ./test_files/records_example.bin: TransactionRecord {
        tx_id: 1000000000000000,
        tx_type: DEPOSIT,
        from_user_id: 0,
        to_user_id: 9223372036854775807,
        amount: 100,
        timestamp: 1633036860000,
        status: FAILURE,
        description: Record number 1,
    }
  In ./test_files/records_example.txt: TransactionRecord {
        tx_id: 1000000000000000,
        tx_type: DEPOSIT,
        from_user_id: 0,
        to_user_id: 9223372036854775807,
        amount: 101,
        timestamp: 1633036860000,
        status: FAILURE,
        description: Record number 1,
    }
Transaction 1000000000000001 differs:
  In ./test_files/records_example.bin: TransactionRecord {
        tx_id: 1000000000000001,
        tx_type: TRANSFER,
        from_user_id: 9223372036854775807,
        to_user_id: 9223372036854775807,
        amount: 200,
        timestamp: 1633036920000,
        status: PENDING,
        description: Record number 2,
    }
  In ./test_files/records_example.txt: TransactionRecord {
        tx_id: 1000000000000001,
        tx_type: TRANSFER,
        from_user_id: 9223372036854775807,
        to_user_id: 9223372036854775807,
        amount: 201,
        timestamp: 1633036920000,
        status: PENDING,
        description: Record number 2,
    }
...
```
## Пример использования
```bash
cargo run -- --file1 ../test_files/records_example.bin --format1 binary --file2 ../test_files/records_example.txt --format2 txt
```