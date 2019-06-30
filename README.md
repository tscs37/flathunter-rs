# Flathunter-rs

Eine Rust Version von [flathunter](https://github.com/tschuehly/flathunter). Nur ImmoScout wird unterstützt.

## Configuration

Flathunter-rs verwendet YAML zur Konfiguration. Erstelle die Datei `flathunter.yml` 
im Arbeitsverzeichnis und setze den folgenden Inhalt:

```yaml
---
evict_after: 1year # Aktuell unbenutzt, 1 Jahr ist Default
run_every: 1h # Anpassen, wie oft prüft Flathunter-rs die Suchen
notify_conf: # Konfigurationen für Notifiers, aktuell nur Stdout (keine Config) und Discord
  webhook:
    display_name: Flathunter-rs
    type: discord
    url: <Discord Webhook URL>
log_level: debug # Log Level, Debug ist Default
urls:
  - <URL einer ImmoScout Suche>
seen: # Hier werden früher gefundene Wohnungen abgelegt
```

## Lizenz

Die Lizenz kann auch in der Datei LICENSE gefunden werden.

Diese Software ist nur für den privaten Gebrauch gestaltet, die kommerzielle Verwendung ist untersagt.

Die Verbreitung des Quellcodes ist nur für die Zwecke der Privaten Verwendung gestattet.

Änderungen am Quellcode müssen an den ursprünglichen Author weitergegeben werden.