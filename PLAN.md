# Plan — Crate `sdl-keybridge`

## Context

Le but du projet est de créer **la pierre de Rosette universelle pour SDL** : une table de correspondance statique qui expose, pour une touche clavier donnée, toutes ses représentations parallèles — scancode physique, keycode logique, glyphe textuel et symbolique, label localisé dans la langue de l'utilisateur — à travers toutes les combinaisons de layouts et de plateformes supportées par SDL.

L'écosystème Rust actuel n'offre aucune crate qui combine les trois dimensions nécessaires :
- **Layout-aware character resolution** (AZERTY/QWERTZ/JCUKEN → bon caractère)
- **i18n des touches nommées** (Escape → Échap/Esc/Escape en FR/DE/EN)
- **Pivot cross-layout via scancode** pour partage de config entre claviers différents

État de l'art et ses limites :
- `SDL_GetKeyName` : "English uppercase only" (documenté explicitement dans SDL3 Best Keyboard Practices)
- `key-names` (HactarCE) : layout-aware mais anglais uniquement, 8 ⭐, maintenance maison
- `pc-keyboard`, `keycode`, `scancode-rs` : conversion de codes, pas de labels, pas d'i18n
- Qt `QKeySequence::toString` : localisé mais Qt-only, non réutilisable hors Qt

Le projet comble ce manque avec trois objectifs :
1. Être **la référence communautaire Rust** pour les représentations parallèles clavier/SDL
2. Couvrir nativement SDL2 et SDL3 (compatibilité via types primitifs `u32`/`u16`)
3. Livrer une feature différentiante : **bridge de bindings cross-layout** via scancode comme pivot universel (ex: partager une config créée sur clavier russe JCUKEN vers un clavier français AZERTY)

Projet communautaire, repo indépendant, dual-licensed MIT / Apache-2.0.

**Répertoire de travail local** : `/home/sylvain/dev/sdl-keybridge`
**Repo GitHub** : `Le-Syl21/sdl-keybridge` (à créer)

---

## Scope

### In-scope v0.1 (Rosette pure)

- **Modèle "BDD"** : table par layout, indexée `(scancode, modifier_level) → { keycode, glyph_textual, glyph_symbolic }`
- **API publique minimale** (pure data lookup, zéro convention) :
  1. `resolve(scancode, mods, layout, locale, style, localizer) → Resolved` — forward lookup, retourne toutes les représentations parallèles en une passe
  2. `scancode_for(keycode, layout) → Option<Scancode>` — reverse lookup
  3. `modifier_label(modifier, platform, locale, style, localizer) → Cow<'static, str>` — label d'un modifier seul (platform-aware : `⌘`/`"Commande"` sur Mac, `⊞`/`"Windows"` sur Windows)
  4. `keycode_from_name(name) → Option<Keycode>` — parsing inverse de noms textuels (`"Escape"` → `SDLK_ESCAPE`)
- **Caps Lock / Num Lock handling correct** dans `resolve()` : Caps applique Shift uniquement aux lettres (comportement OS-standard) ; NumLock bascule la sémantique keypad. Pas optionnel, c'est du *correctness*.
- **`LabelStyle { Textual, Symbolic }`** : paramètre du `resolve()` et `modifier_label()`. `Symbolic` retourne les glyphes Unicode (`↑↓←→⇧⌘⌃⌥⇥⏎`) ; `Textual` retourne les mots (`"Up"` / `"Haut"` / `"Command"` / `"Commande"`). Consumer choisit son style selon son UI.
- **Tous les layouts CLDR** (~50+) embarqués depuis [Unicode CLDR](https://github.com/unicode-org/cldr), build.rs génère les tables statiques à partir du XML LDML (tr35-keyboards). Layouts platform-spécifiques encodés dans l'ID : `mac/fr-t-k0-azerty`, `windows/fr-t-k0-azerty`, etc.
- **Noms de layouts BCP 47 + extension CLDR `-t-k0-`** : `fr-t-k0-azerty`, `de-t-k0-qwertz`, `en-US-t-k0-qwerty`, `ru-t-k0-jcuken`, `en-US-t-k0-dvorak`, etc.
- **i18n touches nommées — trait pluggable + features par locale** :
  - Trait `KeyLocalizer` dans le core
  - 26 features Cargo (`fr`, `de`, `es`, `ja`, `zh-hans`, `zh-hant`, `ru`, `ar`, …) chacune activant un module `crate::locales::<code>::<Lang>Localizer`
  - Chaque module est du **Rust pur** (pas de TOML, pas de codegen) : un `match key_id { ... }` avec les ~30 touches nommées (Escape, Enter, Shift L/R, Ctrl L/R, Alt L/R, AltGr, GUI L/R, arrows, F-keys, Home/End/PageUp/PageDown, Insert/Delete, PrintScreen/Pause/ScrollLock/NumLock/CapsLock, Menu/Application, Tab, Space, Backspace)
  - `default = ["en"]` (anglais garanti)
  - Feature `all-locales` agrégée
  - `MultiLocalizer` auto-registre les localisateurs compilés via `#[cfg(feature = ...)]`
- **Compatibilité SDL2 + SDL3 native, sans feature flag** : l'API prend `u32` (scancode) et `u16` (keymod bitmask). Valeurs numériques identiques à 95% entre SDL2 et SDL3. **Aucune feature `sdl2` / `sdl3`** — une seule API gère les deux. Une table `LEGACY_KEYS` documente les 12 constantes SDL2 supprimées en SDL3 (WWW, MAIL, CALCULATOR, COMPUTER, BRIGHTNESS*, DISPLAYSWITCH, KBDILLUM*, APP1/2) pour que les consommateurs SDL2 n'aient pas de surprise.
- **Types publics** : `Scancode(u32)`, `Keycode(u32)`, `KeyMod(u16)`, `ModifierLevel` (Base/Shift/AltGr/ShiftAltGr), `NamedKey` (enum ~200 variants alignés sur taxonomie SDL3), `Resolved`, `LabelStyle`, `Platform`, `Modifier`
- **License dual MIT / Apache-2.0**, hébergement `Le-Syl21/sdl-keybridge`

### Out-of-scope v0.1 (reportés)

- Feature gating per-layout (pour l'instant tous embarqués, ~100KB statique acceptable) → v0.3 si besoin

### Non-sujets (gratuits via les données CLDR)

- **Variants hardware (ISO/ANSI/JIS/ABNT2/KS)** : les layouts CLDR sont hardware-spécifiques par design. `fr-t-k0-azerty` encode implicitement ISO 105, `en-US-t-k0-qwerty-ansi` vs `-iso` sont deux layouts distincts, JIS est implicite dans `ja-t-k0-*`. On importe les layouts tels quels, la variante hardware vient avec le nom. Zéro code spécifique.

### Philosophie : "Rosette, pas Champollion"

La crate est une **table de correspondance statique** (Rosette). Elle expose des données parallèles (scancode ↔ keycode ↔ glyph_en ↔ glyph_local) et des labels individuels (par touche, par modifier, par plateforme, par locale). Elle **n'interprète pas** et **ne package pas** de conventions de présentation.

Tout ce qui relève de l'interprétation (formatter un combo `Ctrl+Shift+A`, sérialiser un binding, rebaser une config d'un layout à un autre) est **Champollion territory** : c'est le rôle des applications consommatrices (frontends d'émulateur, IDEs, launchers, rebind UIs, etc.), qui **utilisent** la Rosette pour accomplir leur tâche.

### Platform-awareness (Apple, Windows, Linux, ChromeOS)

Les layouts CLDR sont platform-spécifiques (`keyboards/3.0/osx-fr-FR.xml`, `windows-fr-FR.xml`, etc.). La Rosette doit savoir, par layout, que :
- Sur macOS, LGUI est rendu `⌘` (symbolique) ou `"Commande"` (textuel FR) / `"Command"` (textuel EN)
- Sur Windows, LGUI est `⊞` ou `"Windows"`
- Sur Linux, LGUI est `◇` ou `"Super"` / `"Meta"`
- Idem pour LALT sur macOS = `⌥`/`"Option"`, etc.

L'ID de layout encode la plateforme : `mac/fr-t-k0-azerty`, `windows/fr-t-k0-azerty`, `linux/fr-t-k0-azerty`, `chromeos/fr-t-k0-azerty`.

```rust
pub enum Platform { Mac, Windows, Linux, ChromeOS, Android }

pub enum LabelStyle { Textual, Symbolic }

pub fn modifier_label(
    m: Modifier,
    platform: Platform,
    locale: &str,
    style: LabelStyle,
    localizer: &impl KeyLocalizer,
) -> Cow<'static, str>;
// (Ctrl, Mac, "fr", Symbolic, …) → "⌃"
// (Gui, Mac, "fr", Textual, …)   → "Commande"
// (Gui, Windows, "fr", Textual)  → "Windows"
// (Gui, Linux, "en", Textual)    → "Super"
```

### Permanent out-of-scope (non-goals)

- **Détection du layout courant** : la crate ne détecte jamais le layout actif de l'OS. C'est à l'appelant de fournir le nom BCP 47. Raison : aucune solution Rust mature ne couvre les 5 OS de SDL (Windows + macOS + Linux + iOS + Android) de manière fiable. `key-names` est incomplet (macOS hardcoded QWERTY), `rdev` avoue ses limites. La crate reste minimaliste sur ce sujet.
- **Features `sdl2` / `sdl3`** : une seule crate couvre les deux via l'API sur types primitifs (`u32`, `u16`). Pas de split par version de SDL.
- **Dead keys / composition / text input** : notre crate travaille au niveau "une touche → un glyphe", jamais au niveau "séquence de touches → caractère composé". Un scancode 47 sur AZERTY FR retourne toujours `^` (glyph littéral, keycode U+02C6), et ce qui se passe ensuite au niveau text input (OS/IME produisant `â` pour `^+a`) est hors du périmètre. Les consommateurs qui veulent de la composition utilisent `SDL_StartTextInput` / IME natif, pas notre crate.
- **Format de combo localisé** (`format_combo()`) : applique une **convention** de présentation (séparateur, ordre des modifiers, symbolique vs textuelle). C'est Champollion. La crate expose les labels individuels via `modifier_label()` et `resolve().glyph_local` ; le downstream assemble avec son format préféré.
- **`translate()` / bridge packagé** : enchaîne `scancode_for` + `resolve`. La crate expose les deux primitives ; le downstream chaîne lui-même en 2 lignes. Un exemple canonique est documenté dans le README, mais pas packagé en fonction.
- **Serialization de binding** (`Binding::serialize / parse / rebase`) : format de config = choix du downstream (INI, JSON, RON, binaire, etc.). La Rosette ne dicte pas le papyrus.
- **Démo graphique** : pas de binaire GUI (egui/winit/…). Les deux CLI `examples/showcase.rs` et `examples/inspect.rs` couvrent la validation et l'exploration manuelle sans ajouter de dépendance lourde.

---

## Architecture

```
sdl-keybridge/
├── Cargo.toml              # 26 features locales + feature "all-locales"
├── build.rs                # parse CLDR XML → tables statiques
├── data/cldr/              # submodule ou fetch au build
├── src/
│   ├── lib.rs              # re-exports + docs
│   ├── scancode.rs         # Scancode(u32), NamedKey enum, const maps
│   ├── keycode.rs          # Keycode(u32), SDLK_* constants, masks
│   ├── keymod.rs           # KeyMod(u16), bitflags SDL_KMOD_*
│   ├── layout.rs           # Layout struct, get_layout(name), static tables
│   ├── resolve.rs          # resolve(), scancode_for(), modifier_label(), keycode_from_name()
│   ├── named_key.rs        # taxonomy SDL3 ~200 variants, stable ids "key_*"
│   ├── localizer.rs        # KeyLocalizer trait + MultiLocalizer + Platform/Modifier/LabelStyle
│   └── locales/
│       ├── en.rs           # feature = "en", default
│       ├── fr.rs           # feature = "fr"
│       ├── de.rs           # feature = "de"
│       ├── es.rs, it.rs, pt.rs, nl.rs, sv.rs, fi.rs
│       ├── pl.rs, cs.rs, sk.rs, tr.rs
│       ├── ru.rs, ar.rs, hi.rs, bn.rs, ur.rs
│       ├── zh-hans.rs, zh-hant.rs, ja.rs, ko.rs
│       ├── id.rs, sw.rs, vi.rs, th.rs
│       └── mod.rs          # dispatch via #[cfg(feature = ...)]
├── tests/
│   ├── api.rs              # tests des 4 fonctions publiques
│   ├── layouts.rs          # round-trips scancode ↔ keycode par layout
│   ├── caps_num.rs         # correctness Caps Lock / Num Lock (letters vs digits vs keypad)
│   ├── platform.rs         # modifier_label platform-aware (Mac/Windows/Linux)
│   └── locales.rs          # chaque locale : no-missing-key + non-empty, Textual & Symbolic
├── examples/
│   ├── showcase.rs         # walk-through non-interactif des 11 scénarios canoniques
│   └── inspect.rs          # inspecteur CLI : layout + (scancode|keycode|name) → Resolved complet
└── LICENSE-MIT, LICENSE-APACHE, README.md, CONTRIBUTING.md
```

### API publique (Rosette pure)

```rust
pub fn resolve(
    scancode: Scancode,
    mods: KeyMod,                      // incl. CAPS / NUM bits, traités correctement
    layout: &str,                      // "mac/fr-t-k0-azerty"
    locale: &str,                      // "fr"
    style: LabelStyle,                 // Textual | Symbolic
    localizer: &impl KeyLocalizer,
) -> Resolved;

pub fn scancode_for(keycode: Keycode, layout: &str) -> Option<Scancode>;

pub fn modifier_label(
    modifier: Modifier,                // Ctrl | Shift | Alt | Gui | AltGr
    platform: Platform,                // Mac | Windows | Linux | ChromeOS | Android
    locale: &str,
    style: LabelStyle,
    localizer: &impl KeyLocalizer,
) -> Cow<'static, str>;

pub fn keycode_from_name(name: &str) -> Option<Keycode>;

pub struct Resolved {
    pub scancode: Scancode,
    pub keycode: Keycode,
    pub glyph_en: Cow<'static, str>,     // style-aware
    pub glyph_local: Cow<'static, str>,  // style-aware
    pub character: Option<char>,
    pub named_key: Option<NamedKey>,
    pub layout: &'static str,
}

pub enum LabelStyle { Textual, Symbolic }
pub enum Platform { Mac, Windows, Linux, ChromeOS, Android }
pub enum Modifier { Ctrl, Shift, Alt, Gui, AltGr }

pub trait KeyLocalizer {
    fn translate(&self, key_id: &str, style: LabelStyle) -> Option<Cow<'static, str>>;
    fn locale(&self) -> &'static str;
}

pub struct MultiLocalizer { /* auto-registered per enabled feature */ }
```

### Usage patterns documentés (README)

Pas de fonction packagée, juste des exemples canoniques :

```rust
// Bridge cross-layout (usage #3) — 2 lignes de chaînage
let sc = sdl_keybridge::scancode_for(keycode_ru, "windows/ru-t-k0-jcuken")?;
let resolved = sdl_keybridge::resolve(sc, KeyMod::NONE, "windows/fr-t-k0-azerty", "fr", Textual, &loc);

// Formater un combo localisé — 3 lignes côté downstream
let ctrl = sdl_keybridge::modifier_label(Modifier::Ctrl, Platform::Linux, "fr", Textual, &loc);
let key = sdl_keybridge::resolve(sc, KeyMod::NONE, layout, "fr", Textual, &loc).glyph_local;
let combo = format!("{}+{}", ctrl, key);  // "Ctrl+A"
```

### Cargo.toml features matrix

```toml
[features]
default = ["en"]
all-locales = ["ar","bn","cs","de","en","es","fi","fr","hi","id","it","ja","ko","nl","pl","pt","ru","sk","sv","sw","th","tr","ur","vi","zh-hans","zh-hant"]
en = []
fr = []
de = []
# ... 23 autres
```

### Source des layouts (build.rs)

- [`unicode-org/cldr`](https://github.com/unicode-org/cldr) `/keyboards/3.0/*.xml`
- `build.rs` lit les XML, produit `OUT_DIR/layouts_gen.rs` avec `static LAYOUTS: phf::Map<&str, LayoutTable>`
- Dep build-only : `quick-xml`, `phf_codegen`
- Pas de submodule git — fetch via `CARGO_MANIFEST_DIR/data/cldr/` check-in ou tarball téléchargé par build.rs (préférence check-in pour reproductibilité)
- License data : Unicode License, compatible MIT/Apache dual

---

## Bootstrap v0 (premières actions après approbation du plan)

Pour éviter de polluer l'implémentation future avec le contexte du répertoire de travail actuel, on amorce le nouveau repo avec **uniquement** 3 fichiers, puis l'utilisateur relance Claude Code dans `/home/sylvain/dev/sdl-keybridge/` pour une session propre.

1. `mkdir -p /home/sylvain/dev/sdl-keybridge`
2. Écrire `/home/sylvain/dev/sdl-keybridge/PLAN.md` — copie exacte du présent plan (pour qu'il voyage avec le projet)
3. Écrire `/home/sylvain/dev/sdl-keybridge/CLAUDE.md` — brief court pour les sessions futures :
   - Positionnement : "pierre de Rosette universelle pour SDL"
   - Philosophie Rosette vs Champollion (1 paragraphe)
   - Résumé des 4 fonctions publiques
   - Règles d'or (pas de feature sdl2/sdl3, pas de détection, pas de dead keys, pas de serialization, pas de format_combo)
   - Pointeur vers `PLAN.md` pour le détail complet
   - Conventions (fmt+clippy avant push, features BCP 47 en noms courts, license dual MIT/Apache)
4. `git init` dans le répertoire
5. **STOP** — ne pas créer Cargo.toml, build.rs, ni aucun code. L'utilisateur relance Claude Code dans le nouveau répertoire pour session propre.

## Critical files (nouvelle repo, à créer — session 2+)

- `sdl-keybridge/Cargo.toml`
- `sdl-keybridge/build.rs`
- `sdl-keybridge/src/lib.rs`
- `sdl-keybridge/src/layout.rs`
- `sdl-keybridge/src/resolve.rs`
- `sdl-keybridge/src/localizer.rs`
- `sdl-keybridge/src/locales/mod.rs`
- `sdl-keybridge/src/locales/en.rs` (référence anglaise, implémentation canonique à cloner pour toute nouvelle langue)
- `sdl-keybridge/src/locales/fr.rs` + 24 autres locales (une par langue ciblée : de, es, it, pt, nl, sv, fi, pl, cs, sk, tr, ru, ar, hi, bn, ur, zh-hans, zh-hant, ja, ko, id, sw, vi, th)
- `sdl-keybridge/tests/api.rs`
- `sdl-keybridge/tests/layouts.rs`
- `sdl-keybridge/tests/locales.rs`
- `sdl-keybridge/examples/showcase.rs`
- `sdl-keybridge/examples/inspect.rs`
- `sdl-keybridge/README.md`
- `sdl-keybridge/LICENSE-MIT`
- `sdl-keybridge/LICENSE-APACHE`
- `sdl-keybridge/data/cldr/*.xml` (check-in depuis unicode-org/cldr)

---

## Verification

### Tests unitaires (par catégorie)

1. **Taxonomie stable** : `cargo test --test layouts` vérifie que `Scancode::A` vaut 4, `Scancode::ESCAPE` vaut 41, `KeyMod::LSHIFT` vaut 0x0001 (identique SDL2/SDL3)
2. **Round-trip layout** : pour chaque layout embarqué, `scancode_for(resolve(sc, mods, layout, …).keycode, layout) == Some(sc)` (à modifiers constants)
3. **Bridge cross-layout (via chaînage)** : exemple reproductible : `scancode_for(RUSSIAN_F, "windows/ru-t-k0-jcuken")` puis `resolve(sc, NONE, "windows/fr-t-k0-azerty", "fr", Textual, …).glyph_local == "Q"` (F russe → position physique A → Q en AZERTY)
4. **Caps Lock / Num Lock correctness** : `resolve(SCANCODE_A, CAPS, …).glyph_local == "A"` ; `resolve(SCANCODE_1, CAPS, …).glyph_local == "1"` (pas `"!"`) ; `resolve(KP_1, NUM, …).glyph_local == "1"` ; `resolve(KP_1, NONE, …).named_key == Some(NamedKey::End)`
5. **Platform-aware modifier labels** : `modifier_label(Gui, Mac, "en", Symbolic, …) == "⌘"` ; `modifier_label(Gui, Windows, "fr", Textual, …) == "Windows"` ; `modifier_label(Alt, Mac, "fr", Textual, …) == "Option"`
6. **Style Textual vs Symbolic** : `resolve(SCANCODE_UP, NONE, …, Symbolic, …).glyph_local == "↑"` ; `resolve(SCANCODE_UP, NONE, …, Textual, "fr", …).glyph_local == "Haut"`
7. **Locale completeness** : chaque `#[cfg(feature = "xx")]` module doit retourner `Some(_)` pour tous les `key_id` nommés dans `NamedKey`, en Textual et Symbolic. Test itère les `key_*` ids sur le `*Localizer`.
8. **Feature matrix CI** : GitHub Actions build avec `default-features = false`, `features = ["en"]`, `features = ["fr"]`, `features = ["all-locales"]` — tous doivent compiler et passer les tests.
9. **keycode_from_name** : round-trip `SDL_GetKeyName` ↔ `keycode_from_name` sur ~30 touches connues (Escape, Tab, F5, Left Shift, a, 1, etc.)

### Démos CLI (`examples/`)

Deux binaires CLI tiennent le rôle de démo, sans dépendance lourde :

- `examples/showcase.rs` — walk-through non-interactif enchaînant les
  11 scénarios canoniques (cross-layout char resolution, fallback
  locale, Caps/Num correctness, modifiers platform-aware, bridge,
  combo). Utile pour valider un build en un coup d'œil.
- `examples/inspect.rs` — inspecteur interactif. Syntaxe :
  `inspect <layout> <scancode|keycode|name> <value> [mods] [locale] [platform]`.
  Sort le `Resolved` complet dans les deux styles, la table
  base/shift/altgr/shift-altgr, tous les `modifier_label` pour la
  plateforme/locale, et rejoue la touche physique sur les 14 autres
  layouts. Remplace le besoin d'une UI graphique pour valider un
  scancode, debugger une locale ou visualiser le bridge.

### Publication checklist

- [ ] `cargo fmt --check` + `cargo clippy --all-features -- -D warnings` clean
- [ ] `cargo test --all-features` pass
- [ ] `cargo doc --all-features` sans warning
- [ ] `cargo publish --dry-run` OK
- [ ] README avec exemples copier-coller des 4 fonctions publiques + les 2 patterns canoniques (bridge, combo)
- [ ] CONTRIBUTING.md décrit le process "ajouter une langue" (copier `en.rs` → `<code>.rs` → ajouter feature + dispatch dans `mod.rs`)
- [ ] Tag `v0.1.0` + release GitHub avec binaire démo
- [ ] Annonce sur r/rust, This Week in Rust, forums SDL/gamedev
