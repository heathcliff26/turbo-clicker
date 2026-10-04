# Changelog

Version v0.2.2-alpha.0
--------------------
Released: 2026-10-04

🚀 Features:
- *(container)* [**breaking**] Update image ghcr.io/heathcliff26/rust-qt-builder to v202609090334

⚙️ Miscellaneous Tasks:
- Add new changelog workflow
- Enforce conventional commits

💼 Other:
- Remove moved builder status badge
- Fix coverprofiles permissions

Version v0.2.1
--------------------
Released: 2026-08-15

💼 Other:
- Move to container repo
- Add note for GNOME dependency
- Install appindicator deps when needed

Version v0.2.0
--------------------
Released: 2026-08-09

🚀 Features:
- *(container)* Update image docker.io/library/rust to v1.97.0
- *(github-release)* Update goreleaser/goreleaser to v2.17.0

🐛 Bug Fixes:
- *(container)* Update image docker.io/library/rust to v1.97.1
- *(github-release)* Update goreleaser/goreleaser to v2.17.1

💼 Other:
- Fix release workflow using old artifact name
- Move renovate and Link Check to tekton
- Remove obsolete Renovate status badge
- Run cargo upgrade

Version v0.1.5
--------------------
Released: 2026-06-01

🚀 Features:
- *(github-release)* Update goreleaser/goreleaser to v2.16.0
- *(container)* Update image docker.io/library/rust to v1.96.0

💼 Other:
- Add make command for installing dependencies
- Fix permissions for coverprofile workflow

Version v0.1.4
--------------------
Released: 2026-03-27

💼 Other:
- Fix binary not linked against qt6

Version v0.1.3
--------------------
Released: 2026-03-01

💼 Other:
- Remove audit workflow

Version v0.1.2
--------------------
Released: 2026-02-01

💼 Other:
- Remove unneeded prepare step in renovate workflow

Version v0.1.1
--------------------
Released: 2026-01-01

💼 Other:
- Install dependencies when release crate
- Exclude test files from coverage
- Check more often if the autoclicker should be exited
- Do not actually move mouse
- Use tempdir library for testing
- Add workflow to audit dependencies

Version v0.1.0
--------------------
Released: 2025-12-17

📚 Documentation:
- Update docs to reflect changes in behaviour

Version v0.0.8
--------------------
Released: 2025-12-16

📚 Documentation:
- Update screenshots

💼 Other:
- Enable signing artifacts in release workflow

Version v0.0.5
--------------------
Released: 2025-09-01

🐛 Bug Fixes:
- *(deps)* Update cargo

Version v0.0.4
--------------------
Released: 2025-08-03

💼 Other:
- Fix style to fluent
- Add step to publish to crates.io
- Update app to use pages instead of popups and Menu
- Add settings page with option to enabled dark mode
- Drop unneeded cargo vendor
- Increase about slint elements size slightly

Version v0.0.3
--------------------
Released: 2025-08-01

💼 Other:
- Use atomic bool instead of mutex
- Fully install app on system
- Remove leading slash for files

Version v0.0.2
--------------------
Released: 2025-07-30

💼 Other:
- Remove unneeded inputs
- Add display for made with slint logo
- Additionally force-reload icons
- Add udev rule for uinput permissions
- Add instructions for installing rpm

Version v0.0.1
--------------------
Released: 2025-07-27

⚙️ Miscellaneous Tasks:
- Trigger on ui or icon changes

💼 Other:
- Add link to latest release for installation instructions

