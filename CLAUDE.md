# sdl-keybridge — CLAUDE.md

## Positionnement

`sdl-keybridge` est **la pierre de Rosette universelle pour SDL** : une table
de correspondance statique qui expose, pour chaque touche clavier, toutes
ses représentations parallèles — scancode physique, keycode logique,
glyphe textuel et symbolique, label localisé — à travers toutes les
combinaisons de layouts et de plateformes supportées par SDL.

**Objectif** : combler le gap Rust où aucune crate ne combine
layout-awareness (AZERTY/QWERTZ/JCUKEN), i18n des touches nommées,
et bridge cross-layout via scancode comme pivot universel.

Projet communautaire, dual-licensed MIT / Apache-2.0.

## Philosophie : Rosette, pas Champollion

La crate est une **table de correspondance statique** (Rosette). Elle
expose des données parallèles et des labels individuels. Elle **n'interprète
pas** et **ne package pas** de conventions de présentation.

Tout ce qui relève de l'interprétation — formatter un combo `Ctrl+Shift+A`,
sérialiser un binding, rebaser une config d'un layout à un autre — est
**Champollion territory** : c'est le rôle des applications consommatrices,
qui **utilisent** la Rosette pour accomplir leur tâche.

## API publique (4 fonctions, Rosette pure)

```rust
// Forward lookup — retourne toutes les représentations parallèles
resolve(scancode, mods, layout, locale, style, localizer) -> Resolved

// Reverse lookup
scancode_for(keycode, layout) -> Option<Scancode>

// Label d'un modifier seul, platform-aware (⌘ sur Mac, ⊞ sur Windows)
modifier_label(modifier, platform, locale, style, localizer) -> Cow<str>

// Parser inverse de SDL_GetKeyName
keycode_from_name(name) -> Option<Keycode>
```

- `LabelStyle { Textual, Symbolic }` : `"Up"` / `"Haut"` vs `"↑"` ;
  `"Command"` vs `"⌘"`. Consumer choisit selon son UI.
- `Platform { Mac, Windows, Linux, ChromeOS, Android }` : pour les
  modifiers (LGUI = `⌘` Mac, `⊞` Windows, `◇` Linux).
- Caps Lock / Num Lock correctement gérés dans `resolve()` (Caps applique
  Shift uniquement aux lettres, NumLock bascule keypad).

## Règles d'or (non-goals, ne pas les enfreindre)

1. **Pas de détection du layout** — l'appelant fournit le nom BCP 47.
   Raison : aucune solution Rust ne couvre les 5 OS SDL de manière fiable.
2. **Pas de features `sdl2` / `sdl3`** — une seule API sur `u32`/`u16`
   gère les deux (valeurs numériques identiques à 95%).
3. **Pas de dead keys / composition / text input** — on travaille au
   niveau "une touche → un glyphe". La compo (`^` + `e` → `ê`) est un
   niveau au-dessus (SDL_StartTextInput / IME OS).
4. **Pas de `format_combo()`** — c'est une convention de présentation.
   Le consumer assemble les labels individuels avec son séparateur.
5. **Pas de `translate()` packagé** — chaînage trivial `scancode_for +
   resolve` en 2 lignes côté consumer, documenté dans le README.
6. **Pas de serialization de binding** — format de config = choix du
   downstream (INI/JSON/RON/binaire).

## Architecture

- `src/lib.rs` — re-exports + docs
- `src/scancode.rs`, `keycode.rs`, `keymod.rs` — types primitifs
- `src/layout.rs` — tables statiques générées au build depuis CLDR
- `src/resolve.rs` — les 4 fonctions publiques
- `src/localizer.rs` — trait `KeyLocalizer` + `MultiLocalizer`
- `src/locales/<code>.rs` — une locale par feature (`fr`, `de`, …)
  en **Rust pur** (pas de TOML, pas de codegen)
- `build.rs` — parse CLDR XML → tables phf
- `data/cldr/` — check-in des XML CLDR

## Cargo features

- `default = ["en"]` — anglais garanti out-of-the-box
- 26 locales feature-gated : `fr`, `de`, `es`, `it`, `pt`, `nl`, `sv`,
  `fi`, `pl`, `cs`, `sk`, `tr`, `ru`, `ar`, `hi`, `bn`, `ur`,
  `zh-hans`, `zh-hant`, `ja`, `ko`, `id`, `sw`, `vi`, `th`
- `all-locales` — agrégée
- **Pas** de features pour SDL version ni détection

## Layouts

- Tous les layouts CLDR (~50+) embarqués via `build.rs`
- Noms BCP 47 + extension CLDR : `fr-t-k0-azerty`, `de-t-k0-qwertz`,
  `ru-t-k0-jcuken`, `en-US-t-k0-qwerty`, `en-US-t-k0-dvorak`
- Platform encodée dans l'ID : `mac/fr-t-k0-azerty`,
  `windows/fr-t-k0-azerty`, `linux/fr-t-k0-azerty`
- Variantes hardware (ISO/ANSI/JIS/ABNT2/KS) implicites dans le nom CLDR

## Conventions

- **Avant tout `git push`** : `cargo fmt --check` et
  `cargo clippy --all-features -- -D warnings` doivent être clean.
- Ajouter une langue = copier `src/locales/en.rs` → `<code>.rs`,
  ajouter la feature dans `Cargo.toml`, wire dans `mod.rs`.
  Procédure détaillée dans `CONTRIBUTING.md`.
- License dual MIT / Apache-2.0 pour toute contribution.

## Démo egui (`examples/egui-demo/`)

Binaire démo visualisant les 4 fonctions publiques sur un vrai clavier.
Il sert **deux rôles** :
1. Outil de test manuel exhaustif (toutes combinaisons layout × locale ×
   plateforme × style)
2. Fallback UX quand la détection automatique d'un layout côté appli
   consommatrice échoue — l'utilisateur peut toujours choisir à la main

**Contrainte UX importante** : les sélecteurs de layout (input ET output,
pour le bridge) doivent être **hiérarchiques à 2 niveaux** — d'abord la
langue (French, German, Russian, …), puis la liste des claviers pour
cette langue (AZERTY, BEPO, Dvorak-FR, …). UX inspirée du keyboard
picker de l'installeur Linux. Les 50+ layouts ne doivent JAMAIS être
affichés en liste flat ; ce serait inutilisable.

Éléments de l'UI :
- Layout input : picker hiérarchique langue → clavier
- Layout output : idem (pour démo du bridge cross-layout)
- Locale UI : dropdown des features compilées
- Platform : Mac / Windows / Linux / ChromeOS
- LabelStyle : Textual / Symbolic
- Capture clavier live → affiche scancode, mods, keycode, glyph_en, glyph_local
- Section "Bridge" : keycode + layout source → résultat côté target

## Spec complète

Voir `PLAN.md` (à la racine du repo) pour le plan détaillé : contexte,
état de l'art, scope v0.1, tests, publication checklist.
