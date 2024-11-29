# 🦀 CrabBoy

CrabBoy to mobilny emulator tworzony w ramach pracy inżynierskiej, łączący potęgę **Rust** i **Flutter** w jednym projekcie. Aplikacja pozwala na emulację przy jednoczesnym zapewnieniu nowoczesnego, responsywnego interfejsu użytkownika.

## ✨ Kluczowe funkcje

- **Wydajny rdzeń emulatora**: napisany w Rust, zapewnia maksymalną wydajność i niskopoziomową kontrolę.
- **Intuicyjny interfejs użytkownika**: opracowany w Flutterze, oferuje nowoczesny design i wysoką responsywność.
- **Flutter-Rust Bridge (FFI)**: efektywne połączenie Rust i Flutter przy użyciu `flutter_rust_bridge`.
- Modułowa architektura: projekt składa się z dwóch głównych komponentów:
  - **`rust_core`**: rdzeń emulatora, napisany w Rust.
  - **`flutter_interface`**: interfejs użytkownika, stworzony w Flutterze.

## 📁 Struktura projektu

```
CrabBoy/
├── rust_core          # Moduł odpowiedzialny za rdzeń emulatora
│   ├── src
│   ├── Cargo.toml
│   └── target
├── flutter_interface  # Moduł odpowiedzialny za interfejs użytkownika
│   ├── lib
│   ├── pubspec.yaml
│   └── build
└── README.md

```

CrabBoy wykorzystuje zaawansowane technologie w celu zapewnienia wydajności i nowoczesności:

- **Język programowania:**
  - Rust (rdzeń emulatora)
  - Dart (interfejs użytkownika)
- **Framework:**
  - Flutter (tworzenie interfejsu użytkownika)
- **Kluczowe biblioteki:**
  - [`flutter_rust_bridge`](https://github.com/fzyzcjy/flutter_rust_bridge): łączenie kodu Rust i Flutter
  - Standardowe biblioteki Rust oraz Dart
- **Narzędzia deweloperskie:**
  - Cargo (menedżer pakietów i narzędzie budowania dla Rust)
  - Flutter CLI (narzędzie do budowy i uruchamiania aplikacji Flutter)

## 🔧 Jak uruchomić projekt?

### 1. Wymagania wstępne

- **Flutter SDK**: [Instalacja Flutter](https://flutter.dev/docs/get-started/install)
- **Rust**: [Instalacja Rust](https://www.rust-lang.org/tools/install)
