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
CrabBoy/ ├── rust_core # Moduł odpowiedzialny za rdzeń emulatora │ ├── bridge_generator # Skrypty generujące most FFI │ │ ├── flutter_rust_bridge_codegen.sh # Skrypt dla platformy ogólnej │ │ └── flutter_rust_bridge_codegen_ios.sh # Skrypt dla iOS │ ├── src # Kod źródłowy rdzenia emulatora │ │ ├── cpu # Moduł CPU │ │ ├── apu # Moduł APU │ │ ├── mmu # Moduł MMU │ │ ├── ppu # Moduł PPU │ │ ├── timer # Moduł timera │ │ ├── joypad # Moduł joypada │ │ ├── bootrom # Moduł bootrom │ │ ├── emulator # Główny moduł emulatora │ │ ├── cartridge # Moduł obsługi cartridge │ │ ├── api.rs # API dla komunikacji między Flutterem a Rust │ │ ├── lib.rs # Główna biblioteka Rust │ │ ├── bridge_generated.io.rs # Wygenerowany mostek I/O │ │ └── bridge_generated.rs # Wygenerowany mostek dla ogólnej platformy │ ├── Cargo.toml # Konfiguracja projektu Rust │ └── target # Pliki wynikowe kompilacji Rust ├── flutter_interface # Moduł odpowiedzialny za interfejs użytkownika │ ├── lib # Kod źródłowy interfejsu w Dart │ ├── pubspec.yaml # Konfiguracja Fluttera │ └── build # Pliki wynikowe kompilacji Fluttera ├── LICENSE # Licencja projektu └── README.md # Dokumentacja projektu

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
