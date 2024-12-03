# 🦀 CrabBoy

CrabBoy to mobilny emulator tworzony w ramach pracy inżynierskiej, łączący potęgę **Rust** i **Flutter** w jednym projekcie. Aplikacja pozwala na emulację przy jednoczesnym zapewnieniu nowoczesnego, responsywnego interfejsu użytkownika.

## ✨ Kluczowe funkcje

- **Wydajny rdzeń emulatora**: napisany w Rust, zapewnia maksymalną wydajność i niskopoziomową kontrolę.
- **Intuicyjny interfejs użytkownika**: opracowany w Flutterze, oferuje nowoczesny design i wysoką responsywność.
- Modułowa architektura: projekt składa się z dwóch głównych komponentów:
  - **`rust_core`**: rdzeń emulatora, napisany w Rust.
  - **`flutter_interface`**: interfejs użytkownika, stworzony w Flutterze.

## 📁 Struktura projektu

```
CrabBoy/
├── rust_core          # Moduł odpowiedzialny za rdzeń emulatora
│   ├── src
│   │   ├── cpu
│   │   ├── ppu
│   │   ├── apu
│   │   ├── mmu
│   │   ├── timer
│   │   ├── joypad
│   │   ├── bootrom
│   │   ├── cartridge
│   │   ├── emulator
│   │   ├── lib.rs
│   │   ├── api.rs
│   │   ├── bridge_generated.io.rs
│   │   └── bridge_generated.rs
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── build_for_android.sh
│   └── build_for_ios.sh
├── flutter_interface  # Moduł odpowiedzialny za interfejs użytkownika
│   ├── lib
│   │   ├── components
│   │   ├── pages
│   │   ├── router
│   │   ├── services
│   │   ├── main.dart
│   │   ├── bridge_definitions.dart
│   │   └── bridge_generated.dart
│   ├── android
│   ├── ios
│   ├── assets
│   ├── pubspec.yaml
│   └── pubspec.lock
└── bridge_generator
    ├── flutter_rust_bridge_codegen.sh
    └── flutter_rust_bridge_codegen_ios.sh
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

Aby uruchomić aplikację CrabBoy, musisz mieć zainstalowane:

- **Flutter SDK**: [Instalacja Flutter](https://flutter.dev/docs/get-started/install)
- **Android Studio** z konfiguracją Android SDK
- **Xcode** dla budowy iOS (tylko na macOS)

### 2. Sprawdzenie środowiska

Użyj komendy `flutter doctor`, aby upewnić się, że Twoje środowisko jest gotowe do uruchamiania aplikacji. Minimalne wymagania to:
[✓] Flutter (Channel stable, 3.24.3)
[✓] Android toolchain - develop for Android devices (Android SDK version 35.0.0)
[✓] Xcode - develop for iOS and macOS (Xcode 15.4)
[✓] Android Studio (version 2023.2)
[✓] VS Code (version 1.90.2)

### 3. Instalacja zależności

Przejdź do katalogu `flutter_interface` i zainstaluj zależności Fluttera:

```bash
cd flutter_interface
flutter pub get
```

### 4. Przygotowanie środowiska uruchomieniowego

Przed uruchomieniem aplikacji upewnij się, że masz podłączony **emulator**, **symulator** lub fizyczne urządzenie do swojego środowiska IDE.

#### **Wgrywanie na emulator Androida**

1. Otwórz Android Studio.
2. Przejdź do **"Device Manager"**.
3. Wybierz **"Create Virtual Device"** i skonfiguruj urządzenie.
4. Kliknij **Start**, aby uruchomić emulator.
5. **Uruchom aplikację na urządzeniu**:
   - W trybie debugowania:
     ```bash
     flutter run
     ```
   - W trybie wydania:
     ```bash
     flutter run --release
     ```

#### **Wgrywanie na symulator iOS**

1. Otwórz Xcode.
2. Przejdź do **"Open Developer Tool" → "Simulator"**.
3. Wybierz odpowiednie urządzenie, aby uruchomić symulator.
4. **Uruchom aplikację na urządzeniu**:
   - W trybie debugowania:
     ```bash
     flutter run
     ```
   - W trybie wydania:
     ```bash
     flutter run --release
     ```
5. **Dodatkowe kroki dla konta deweloperskiego Apple**:

- W Xcode otwórz projekt Fluttera znajdujący się w katalogu `flutter_interface/ios/Runner`.
- Ustaw swój **team deweloperski** w ustawieniach projektu.
- Zbuduj aplikację i uruchom ją na urządzeniu.

#### **Wgrywanie na fizyczne urządzenie**

- **Dla urządzeń z Androidem**:

  1. **Włącz tryb debugowania USB na urządzeniu**:
     - Otwórz ustawienia urządzenia.
     - Przejdź do sekcji **Informacje o telefonie** i kilkukrotnie kliknij w **Numer kompilacji**, aż odblokujesz opcje deweloperskie.
     - Przejdź do **Opcji deweloperskich** i włącz **Debugowanie USB**.
  2. **Podłącz urządzenie do komputera** za pomocą kabla USB.
  3. **Sprawdź połączenie**:
     - Użyj polecenia:
       ```bash
       flutter devices
       ```
     - Powinieneś zobaczyć swoje urządzenie na liście podłączonych urządzeń.
  4. **Uruchom aplikację na urządzeniu**:
     - W trybie debugowania:
       ```bash
       flutter run
       ```
     - W trybie wydania:
       ```bash
       flutter run --release
       ```

- **Dla urządzeń z iOS**:
  1. **Podłącz urządzenie do komputera** za pomocą kabla.
  2. **Sprawdź połączenie**:
     - Otwórz Xcode i upewnij się, że urządzenie jest widoczne w zakładce **Devices and Simulators**.
  3. **Włącz tryb deweloperski na urządzeniu**:
     - Jeśli urządzenie prosi o zaufanie komputerowi, zaakceptuj to.
     - W razie potrzeby przejdź do ustawień urządzenia i aktywuj tryb deweloperski.
  4. **Uruchom aplikację na urządzeniu**:
     - W trybie debugowania:
       ```bash
       flutter run
       ```
     - W trybie wydania:
       ```bash
       flutter run --release
       ```
  5. **Dodatkowe kroki dla konta deweloperskiego Apple**:
     - W Xcode otwórz projekt Fluttera znajdujący się w katalogu `flutter_interface/ios/Runner`.
     - Ustaw swój **team deweloperski** w ustawieniach projektu.
     - Zbuduj aplikację i uruchom ją na urządzeniu.

## 🧑‍💻 Autor

**Piotr Zapała**

CrabBoy został stworzony jako projekt inżynierski, łączący nowoczesne technologie Rust i Flutter. Głównym celem było opracowanie wydajnego emulatora Gameboya z intuicyjnym interfejsem użytkownika.

---

## 📄 Licencja

Ten projekt jest licencjonowany na warunkach licencji MIT.  
Możesz dowiedzieć się więcej o licencji w pliku [LICENSE](./LICENSE).

---

Dziękuję za zainteresowanie projektem CrabBoy! 🦀
