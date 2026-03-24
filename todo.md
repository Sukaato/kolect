# Kolect — TODO

## Onboarding screen

- [x] Detect language automatically from Android/desktop device
- [x] If language cannot be detected, ask the user to pick one (FR/EN) → `setting.store`
- [x] Ask the user to choose a theme (Light / Dark / System) → `setting.store`
- [x] Ask whether to include photocard count in the display → `setting.store`
- [x] Ask whether to collect items outside the user's region
  - [x] If yes, persist `includeExclusiveItems: boolean` → `setting.store` + Rust backend
- [x] Persist `onboardingDone: boolean` flag once onboarding is completed so it is never shown again
- [x] Add route `/onboarding` + guard in `Startup.vue` (redirect if `!onboardingDone`)
- [x] Add all i18n keys for the onboarding flow (`screen.onboarding.*`)


## Theme transitions

- [ ] Investigate View Transitions API feasibility on WebView + Tauri
- [ ] Implement smooth theme transition animation using View Transitions API if supported

---

## Home & Collection filters

- [x] Replace sort chips with a text input filter (name search)
- [x] Add an agency dropdown filter
  - [x] On Collection screen: only list agencies that have at least one owned item
- [x] Make common component for Home and Collection

---

## Image storage

- [ ] Evaluate and pick a hosting solution (Bunny CDN, Cloudflare Images, Backblaze B2 + CF…)
- [ ] Store relative image URLs in the DB, prefix with a configurable base URL
- [ ] Update all image references across the app to use the chosen CDN

---

## Performance — cursor-based pagination

- [ ] Replace limit/offset pagination with cursor-based pagination
- [ ] Migrate from UUID v4 to UUID v7 (time-ordered, no extra column needed)
- [ ] Update Rust repositories and DTOs accordingly
- [ ] Update frontend stores (`collection.store`, `dataset.store`) to handle cursor pagination

---

## Mobile UX

### Manual item entry

- [ ] Design a stepper flow for adding an item manually to the collection
- [ ] Implement barcode scanner support (`tauri-plugin-barcode-scanner`)
- [x] Clean up backend before wiring the UI (commands, services, repositories)

### Profile screen

- [ ] Create a dedicated Profile screen
- [ ] Move Settings screen behind a gear icon in the Profile header
- [ ] Define Profile screen content
