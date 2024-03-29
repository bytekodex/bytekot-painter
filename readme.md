# Bytekot painter

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=bytekodex_bytekot-painter&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=bytekodex_bytekot-painter)

Библиотека-художник изображений для bytekot телеграм бота, для генерации изображения из текстового представления байткода с подсветкой.
Так же планируется использование в javap-viewer.

Суть проста, библиотека экспоузит наружу (пока что) один единственный метод, `paint` (и `free_paint`) принимающий два аргумента "_си строки_", `input` и `path`, первый собственно сам jvm байткод,
второй это путь, куда будет сохранена картинка (в зависимости от ОС, относительный и абсолютный путь, зависят от прямого слеша или точки с прямым слешем).

После сохранения картинки, библиотека отдает управление обратно с сишной строкой (либо это будет путь до картинки, либо текстовая ошибка).

В дальнейшем, я поменяю интерфейс на возврат int кода, для ошибок. (PR приветствуются)

## Баги и улучшения

- Любые баг-фиксы, улучшения приветствуются :) https://github.com/bytekodex/bytekot-painter/pulls
- Любые баг репорты, предложения, так же приветствуются https://github.com/bytekodex/bytekot-painter/issues

## Готовый образ

Готовый образ (`linux/amd64`) доступен в официальном docker hub по имени `bytecodex/bytekot-painter`

```shell
docker push bytecodex/bytekot-painter:v1.0.1
```

Для multi-stage сборки, аналогичным образом (он содержит только файл статической библиотеки)

```dockerfile
FROM bytecodex/bytekot-painter:v1.0.1 as bytekot-painter
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
java -jar rust-antlr.jar -Dlanguage=Rust antlr/JBytecodeParser.g4 antlr/JBytecodeLexer.g4 -o /src/antlr/
```

2. Соберем библиотеку (по умолчанию генерируется статическая библиотека `.lib` на винде и `.a` на linux)

```shell
cargo build --release
```

3. Скомпилированная библиотека будет в `./target/release` с названием `bytekot_painter.lib` или на linux `libbytekot_painter.a`

### Docker

![](/nothing/docker-meme.jpg)

```shell
docker build -t bytecodex/bytekot-painter:v1.0.1 .
```

## Пример результата

Входные параметры: Байткод `java.util.jar.JarEntry` полученный с `javap` с аргументом `-c`.

Результат:

![](/nothing/snapshot-result.png)

## Варианты ошибок

| Ключ                     | Описание                                                                             |
|--------------------------|--------------------------------------------------------------------------------------|
| `too-large-image`        | Изображение слишком большое (высота больше `8192` или `height * width` > `27852800`) |
| `image-encoding-failure` | Не удалось создать растеризатор                                                      |
| `file-creation-failure`  | Не удалось создать файл                                                              |
| `file-writing-failure`   | Не удалось записать в файл                                                           |

## Использованные технологии

- [Rust](https://github.com/rust-lang/rust), лицензирован с [MIT](https://github.com/rust-lang/log/blob/master/LICENSE-MIT)
  и [Apache 2.0](https://github.com/rust-lang/log/blob/master/LICENSE-APACHE)
- [Antlr4](https://github.com/antlr/antlr4), лицензирован с [BSD 3](https://github.com/antlr/antlr4/blob/dev/LICENSE.txt)
- [Skia](https://github.com/google/skia), лицензирован с [BSD 3](https://github.com/google/skia/blob/main/LICENSE)
- [Jetbrains AI](https://www.jetbrains.com/ai/), использован для названий коммитов