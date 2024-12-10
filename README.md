# 🦀 CrabBoy

CrabBoy to mobilny emulator konsoli GameBoy tworzony w ramach pracy inżynierskiej, łączący potęgę **Rust** i **Flutter** w jednym projekcie. Aplikacja pozwala na emulację przy jednoczesnym zapewnieniu nowoczesnego, responsywnego interfejsu użytkownika.

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

Użyj komendy `flutter doctor`, aby upewnić się, że Twoje środowisko jest gotowe do uruchamiania aplikacji. Poniżej lista kluczowych narzędzi wraz z ich wersjami:

- **Flutter (3.24.3)**: Kanał stable
- **Dart (3.5.3)**: Wersja środowiska Dart
- **Android SDK (35.0.0)**: W pełni skonfigurowany z zaakceptowanymi licencjami
- **Xcode (15.4)**: Z CocoaPods w wersji 1.16.0
- **Android Studio (2023.2)**: Z Java w wersji **OpenJDK Runtime Environment 17.0.9**
- **VS Code (1.95.3)**: Z rozszerzeniem Flutter w wersji 3.102.0

> **Notatka**: Podane wersje narzędzi są tymi, z których korzystano podczas tworzenia projektu. Inne wersje mogą działać, ale nie są gwarantowane.

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

## 🐞 Problemy z budowaniem projektu i ich rozwiązania

Podczas testowania projektu na urządzeniu z Linuxem dla aplikacji na Androida mogą wystąpić następujące błędy:

### **1. Błąd: Niekompatybilna wersja Gradle z wersją JDK**

**Objawy:**
Your project's Gradle version is incompatible with the Java version that Flutter is using for Gradle.
**Rozwiązanie:**
Otwórz plik `android/gradle/wrapper/gradle-wrapper.properties` i zmień wartość `distributionUrl` na kompatybilną z używaną wersją JDK:

```properties
distributionUrl=https\://services.gradle.org/distributions/gradle-8.4-all.zip
```

### **2. Błąd: Nieaktualne ustawienia kompilacji Java**

**Objawy:**
Execution failed for task ':path_provider_android:compileDebugJavaWithJavac'.
**Rozwiązanie:**
Otwórz plik `android/app/build.gradle` i zaktualizuj ustawienia Java:

```properties
android {
    ndkVersion = "25.1.8937393"

    compileOptions {
        sourceCompatibility JavaVersion.VERSION_17
        targetCompatibility JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}
```

### **3. Błąd: Niekompatybilna wersja wtyczki Android Gradle Plugin**

**Objawy:**
Could not resolve all files for configuration ':path_provider_android:androidJdkImage'.
**Rozwiązanie:**
Otwórz plik `android/settings.gradle` i zaktualizuj ustawienia:

```properties
id "com.android.application" version "8.3.1" apply false
```

## 🛠️ Generowanie statycznych i dynamicznych bibliotek Rust

W przypadku konieczności wygenerowania bibliotek Rust dla platform **iOS** oraz **Android**, przygotowano dwa skrypty w katalogu `rust_core`. Aby z nich skorzystać, wykonaj poniższe kroki:

### 1. Wymagania wstępne

1. Zainstaluj **Rust** oraz narzędzia pomocnicze:
   ```bash
   rustup install stable
   cargo install cargo-ndk
   cargo install cargo-lipo
   ```
2. Dodaj odpowiednie targety Rust:

**Dla iOS**:

```bash
rustup target add aarch64-apple-ios x86_64-apple-ios
```

**Dla Androida**:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### 2. Generowanie bibliotek

- **Dla Androida**:

  1. Przejdź do katalogu `rust_core`:

  ```bash
    cd rust_core
  ```

  2. Uruchom skrypt:

  ```bash
    ./build_for_android.sh
  ```

  3. Biblioteki zostaną wygenerowane i automatycznie przeniesione do folderu `flutter_interface/android/app/src/main/jniLibs`.

- **Dla iOS**:
  1. Przejdź do katalogu `rust_core`:
  ```bash
    cd rust_core
  ```
  2. Uruchom skrypt:
  ```bash
    ./build_for_ios.sh
  ```
  3. Biblioteka zostanie wygenerowana i automatycznie przeniesiona do folderu `flutter_interface/ios/Runner`.

### 3. Konfiguracja w Xcode (dla iOS)

Po wygenerowaniu biblioteki dla iOS należy dodatkowo skonfigurować projekt w Xcode:

- **Dodaj bibliotekę do projektu**:

  1. Przejdź do katalogu `flutter_interface`:

  ```bash
    cd flutter_interface
  ```

  2. Uruchom projekt w środowisku Xcode:

  ```bash
    open ios/Runner.xcworkspace
  ```

  2. Przejdź do **Build Phases** → **Link Binary With Libraries** dodaj plik `librust_core.a` z folderu `flutter_interface/ios/Runner`.

- **Ustawienia dla symulatora**:
  1. Jeśli planujesz korzystać z symulatora iOS, musisz dodać odpowiednie ustawienia w sekcji **Build Settings**:
  - Znajdź pole **Excluded Architectures**.
  - Dla symulatora ustaw wartość **arm64** w **Profile**, **Debug** i **Release**.

#### 4. Finalizacja konfiguracji:

Po wykonaniu powyższych kroków projekt będzie gotowy do użycia zarówno na urządzeniach fizycznych, jak i w symulatorze.

## 🧑‍💻 Autor

**Piotr Zapała**

CrabBoy został stworzony jako projekt inżynierski, łączący nowoczesne technologie Rust i Flutter. Głównym celem było opracowanie wydajnego emulatora Gameboya z intuicyjnym interfejsem użytkownika.

## 📄 Licencja

Ten projekt jest licencjonowany na warunkach licencji MIT.  
Możesz dowiedzieć się więcej o licencji w pliku [LICENSE](./LICENSE).

Dziękuję za zainteresowanie projektem CrabBoy! 🦀
