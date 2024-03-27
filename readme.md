# Bytekot painter

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)

Библиотека-художник изображений для bytekot телеграм бота, для генерации изображения из текстового представления байткода с подсветкой.
Так же планируется использование в javap-viewer.

Суть проста, библиотека экспоузит наружу (пока что) один единственный метод, `paint` (и `free_image_data`) принимающий аргумент "_си строки_", `input` и возвращает структуру с данными, первый собственно сам jvm байткод,
второй это путь, куда будет сохранена картинка (в зависимости от ОС, относительный и абсолютный путь, зависят от прямого слеша или точки с прямым слешем).

## Баги и улучшения

- Любые баг-фиксы, улучшения приветствуются :) https://github.com/bytekodex/bytekot-painter/pulls
- Любые баг репорты, предложения, так же приветствуются https://github.com/bytekodex/bytekot-painter/issues

## Готовый образ

Готовый образ (`linux/amd64`) доступен в официальном docker hub по имени `bytecodex/bytekot-painter`

```shell
docker push bytecodex/bytekot-painter:v1.1.3
```

Для multi-stage сборки, аналогичным образом (он содержит только файл динамической библиотеки и заголовочный файл (`bytekot_painter.dll/so`,`bytekot_painter.h`))

```dockerfile
FROM bytecodex/bytekot-painter:v1.1.3 as bytekot-painter
```

## Сборка

### Bare Metal

#### Требования:

- JDK (Или JRE) 11 версии и выше.
- Rust 1.76.0 и выше.

#### Шаги

0. Сфетчите гитовые сабмодули

```shell
git submodule init && git submodule update
```

1. Сгенерируем antlr лексер и парсер

```shell
java -jar rust-antlr.jar -Dlanguage=Rust antlr/JBytecodeParser.g4 antlr/JBytecodeLexer.g4 -o ./src/antlr/
```

2. Соберем библиотеку (по умолчанию генерируется динамическая библиотека `.dll` на винде и `.so` на linux)

```shell
cargo build --release
```

3. Скомпилированная библиотека будет в `./target/release` с названием `bytekot_painter.dll` или на linux `libbytekot_painter.so`

### Docker

![](/nothing/docker-meme.jpg)

```shell
docker build -t bytecodex/bytekot-painter:v1.1.3 .
```

## Пример результата

Входные параметры: Байткод `java.util.jar.JarEntry` полученный с `javap` с аргументом `-c`.

Результат:

![](/nothing/snapshot-result.png)

## API

API библиотеки очень простой, всего две функции, собственно, вот хедер для наглядности.

```c
typedef struct ImageResult {
  const unsigned char *data;
  uintptr_t len;
  int status;
} ImageResult;

struct ImageResult paint(const char *input);

void free_image_data(unsigned char *ptr, uintptr_t len);
```

`paint` принимает обычную си строку с байткодом для обработки и всегда возвращает структуру `ImageResult`.

`data` Это указатель на данные массива, которые в последствии можно записать в файл, массив этих байтов является форматом `png`.

`len` Это длина массива.

`status` Это статус операции, ниже коды ошибок представленные кодом: 

```rust
const ERR_SUCCESS: i32 = 0;
const ERR_TOO_LARGE_IMAGE: i32 = -1;
const ERR_RASTER_CREATION_FAILURE: i32 = -2;
const ERR_IMAGE_ENCODING_FAILURE: i32 = -3;
```

_Отдельное пояснение, `ERR_TOO_LARGE_IMAGE` возвращается если высота больше `8192` или `height * width` > `27852800`._

Соответственно для того чтобы не создавать утечку памяти, есть `free_image_data` функция, которая принимает в себя указатель на массив и его длинну.

## Использованные технологии

- [Rust](https://github.com/rust-lang/rust), лицензирован с [MIT](https://github.com/rust-lang/log/blob/master/LICENSE-MIT)
  и [Apache 2.0](https://github.com/rust-lang/log/blob/master/LICENSE-APACHE)
- [Antlr4](https://github.com/antlr/antlr4), лицензирован с [BSD 3](https://github.com/antlr/antlr4/blob/dev/LICENSE.txt)
- [Skia](https://github.com/google/skia), лицензирован с [BSD 3](https://github.com/google/skia/blob/main/LICENSE)
- [Jetbrains AI](https://www.jetbrains.com/ai/), использован для названий коммитов