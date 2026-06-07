# RustTermView

Narzędzie CLI do wyświetlania obrazów i animacji GIF bezpośrednio w terminalu, napisane w Rust.

## Funkcje

- **4 tryby wyświetlania**: Truecolor, Halfblock, Grayscale, ASCII Art
- **Statyczne obrazy**: PNG, JPEG, BMP
- **Animacje GIF**: z równoległym pre-renderowaniem klatek (rayon)
- **Slideshow**: automatyczny pokaz slajdów z katalogu
- **Full-res**: wyświetlanie w pełnej rozdzielczości (scrollowalne)
- Automatyczne dopasowanie do rozmiaru terminala z korekcją proporcji

## Struktura projektu

```
RustTermView/
├── Cargo.toml              # zależności: image, crossterm, clap, rayon
├── README.md
├── assets/
│   ├── pictures/           # przykładowe obrazy (jpeg, jpg)
│   └── gifs/               # przykładowe animacje GIF
└── src/
    ├── main.rs             # punkt wejścia, routing do trybów pracy
    ├── args.rs             # parsowanie argumentów CLI (clap)
    ├── loader.rs           # wczytywanie plików (obrazy, GIF, katalogi)
    ├── transform.rs        # skalowanie obrazu, konwersja do grayscale/ASCII
    ├── render.rs           # generowanie sekwencji ANSI escape codes
    └── terminal.rs         # zarządzanie trybem raw, obsługa klawiatury
```

### Co robi każdy moduł

| Moduł | Odpowiedzialność |
|-------|------------------|
| `args.rs` | Definiuje argumenty CLI (`--mode`, `--fps`, `--fullres`, itd.) za pomocą biblioteki `clap` |
| `loader.rs` | Otwiera i dekoduje pliki graficzne. Dla GIF-ów wyciąga wszystkie klatki z opóźnieniami |
| `transform.rs` | Skaluje obraz do rozmiaru terminala z korekcją proporcji (znaki terminala są 2× wyższe niż szerokie). Zawiera też konwersję RGB → jasność |
| `render.rs` | Zamienia piksele na tekst ANSI: kolorowe spacje (truecolor), znaki ▀ (halfblock), skala szarości lub znaki ASCII |
| `terminal.rs` | Przełącza terminal w tryb raw (natychmiastowa reakcja na klawisze), chowa kursor, obsługuje wyjście (`q` / `Ctrl+C`) |
| `main.rs` | Łączy wszystko — decyduje który tryb uruchomić i wykonuje główną pętlę programu |

### Przepływ danych

```
Argumenty CLI → wczytanie pliku → skalowanie → renderowanie do ANSI → wyświetlenie w terminalu
```

## Wymagania

- **Rust** (edition 2021)
- Terminal wspierający ANSI escape codes (praktycznie każdy nowoczesny terminal na Linuxie/macOS)

## Kompilacja

```bash
cargo build --release
```

## Instrukcja obsługi

### Wyświetlanie obrazu

```bash
cargo run --release -- assets/pictures/kobe.jpeg
```

### Zmiana trybu wyświetlania

```bash
# Truecolor (domyślny) — pełne kolory 24-bit
cargo run --release -- assets/pictures/kobe.jpeg --mode truecolor

# Halfblock — podwójna rozdzielczość pionowa, znak ▀
cargo run --release -- assets/pictures/kobe.jpeg --mode halfblock

# Skala szarości
cargo run --release -- assets/pictures/kobe.jpeg --mode grayscale

# ASCII Art
cargo run --release -- assets/pictures/kobe.jpeg --mode ascii
```

### Odtwarzanie animacji GIF

```bash
cargo run --release -- assets/gifs/oppenheimer.gif
```

Zmiana szybkości odtwarzania:

```bash
cargo run --release -- assets/gifs/oppenheimer.gif --fps 24
```

### Slideshow (pokaz slajdów z katalogu)

```bash
cargo run --release -- assets/pictures/
```

### Pełna rozdzielczość (scrollowalna)

```bash
cargo run --release -- assets/pictures/kobe.jpeg --fullres
```

### Ręczne ustawianie rozmiaru

```bash
cargo run --release -- assets/pictures/kobe.jpeg -W 120 -H 40
```

### Zamykanie programu

Naciśnij **`q`** lub **`Ctrl+C`**.

## Użyte biblioteki

| Biblioteka | Wersja | Do czego |
|------------|--------|----------|
| `image` | 0.25 | Dekodowanie obrazów (PNG, JPEG, BMP, GIF), skalowanie |
| `crossterm` | 0.27 | Obsługa terminala: raw mode, kursor, klawiatura |
| `clap` | 4.4 | Parsowanie argumentów wiersza poleceń |
| `rayon` | 1 | Równoległe pre-renderowanie klatek GIF |
