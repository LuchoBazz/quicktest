<h1 align="center">🦉 Quick Test CLI</h1>

<p align="center">Interfaz de línea de comandos (CLI) para pruebas de estrés para programación competitiva</p>

[![Current Crates.io Version](https://img.shields.io/crates/v/quicktest.svg)](https://crates.io/crates/quicktest) [![Quick Test](https://circleci.com/gh/LuisMBaezCo/quicktest.svg?style=shield)](https://app.circleci.com/pipelines/github/LuisMBaezCo/quicktest) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![](https://img.shields.io/crates/d/quicktest)](https://crates.io/crates/quicktest)

## Documentación

**⚠️ Nota:** _Quick Test CLI_ está actualmente por debajo de la versión `v1.0.0`, por lo que puede contener errores, puede informar cualquier error [aquí](https://github.com/LuisMBaezCo/quicktest/issues).

Lea esto en otros idiomas: [_English_](./../README.md)

**Tabla de contenido**

- [Introducción](#introducción)
- [Instalación](#instalación)
- [Lenguajes admitidos](#lenguajes-admitidos)
- [Licencia](#licencia)

## Introduction

Quick Test CLI es un proyecto para realizar pruebas de estrés en concursos de programación competitivos de forma fácil y rápida, centrándose únicamente en el concurso.

Actualmente, Quick Test CLI admite tres tipos de pruebas que se enumeran a continuación:

* **Detectar casos con TLE:** Verifique que el tiempo de ejecución del código no exceda lo permitido, utilizando un generador aleatorio para múltiples casos de prueba.
    * **Ejemplo:**
        ```shell
        quicktest tle --target-file=”main.cpp” --gen-file=”gen.cpp” --timeout=2000 --test-cases=1000
        ```

* **Verifique la correctitud del código comparandola con una versión más lenta:** Verifica que el código no tenga respuestas incorrectas para algunos casos de prueba, utilizando un generador aleatorio y una versión más lenta qué es seguro qué es totalmente correcta con la cual se van a comparar las respuestas.
    * **Ejemplo:**
        ```shell
        quicktest cmp --target-file=”main.cpp” --correct-file=”correct.cpp” --gen-file=”gen.cpp” --timeout=2000 --test-cases=1000
        ```

* **Verifique la correctitud del código usando un script de verificación:** Similar a la anterior, esta prueba verifica que el código no tenga una respuesta incorrecta para algunos casos de prueba usando un script de verificación porque puede haber muchas respuestas correctas.
    * **Ejemplo:**
        ```shell
        quicktest check --target-file=”main.cpp” --checker-file=”correct.cpp” --gen-file=”gen.cpp” --timeout=2000 --test-cases=1000
        ```

### Instalación

Si ya tiene Rust en su sistema:

```sh
cargo install quicktest
```

Si no tiene rust instalado en su sistema, el siguiente comando instalará Rust y la CLI a la vez:

Para Shell (Linux, Mac):
```sh
curl https://sh.rustup.rs -sSf | sh  && cargo install quicktest
```

## Lenguajes admitidos

| Lenguaje           |       Versión          |
|--------------------|------------------------|
| C++                | -std=c++17             |
| Python             | Version 3              |


## Licencia
Licenciado bajo:
* Licencia MIT ([LICENSE-MIT](https://github.com/LuisMBaezCo/quicktest/blob/main/LICENSE) o https://opensource.org/licenses/MIT)