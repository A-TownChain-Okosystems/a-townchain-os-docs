# 🤖 Agent Policy — Sync- und Entwicklungs-Protokoll

> **Verbindlich für:** Aurora (A-Town Ecosystem Brain)
> **Version:** 1.0.0 | **Stand:** 2026-06-12
> **Gilt für:** Alle Sync-Läufe, Code-Änderungen, Wiki-Updates, Issue-Erstellungen

---

## OBERSTE REGEL: REALITY-CHECK

Der Agent darf **niemals** behaupten, eine Aktion ausgeführt zu haben,
ohne einen bestätigten API-Response erhalten zu haben.

### Ausführungsstatus (immer explizit angeben)

| Status | Symbol | Bedeutung |
|--------|--------|-----------|
| AUSGEFÜHRT | ✅ | API-Call erfolgreich, Response mit ID/SHA bestätigt |
| VORBEREITET | 🔄 | Content fertig, Push noch ausstehend |
| GEPLANT | 📋 | Logik definiert, kein API-Call gemacht |
| SIMULIERT | 🔲 | Nur lokal berechnet, kein externer Effekt |
| FEHLGESCHLAGEN | ❌ | API-Call gemacht, Fehler-Response erhalten |
| UNKLAR | ⚠️ | Response mehrdeutig, Verifizierung nötig |

### Reality-Check-Checkliste vor jeder Aktion

```
Vor GitHub Push:
  □ Token gültig? (401 = nein)
  □ Repo existiert? (404 = nein)
  □ Datei existiert → SHA vorhanden?
  □ Content tatsächlich geändert?
  □ Response enthält commit.sha?

Vor Issue-Erstellung:
  □ Repo erreichbar?
  □ Kein Duplikat (Titel-Check)?
  □ Response enthält number?

Vor E-Mail-Versand:
  □ Token gültig?
  □ Response enthält id (Gmail) oder HTTP 202 (Outlook)?

Vor Notion-Update:
  □ Token gültig?
  □ Datenbank-ID bekannt?
  □ Response enthält id?
```

---

## CODE ↔ DOKUMENTATION ABGLEICH

Bei **jeder** Änderung — in dieser Reihenfolge:

```
1. Code analysieren      → Was existiert tatsächlich?
2. Architektur prüfen    → Layer-Zuordnung korrekt?
3. Wiki analysieren      → Kapitel vorhanden und aktuell?
4. Roadmap prüfen        → Sprint-Zuordnung stimmt?
5. Standards prüfen      → ATC-Nummer referenziert?
6. Issues prüfen         → Ticket vorhanden?
```

**Sechs Abgleichsfragen (alle müssen "nein" sein vor Release):**

```
□ Dokumentation ohne Implementierung?  → Status: DOCUMENTED (Warnung)
□ Implementierung ohne Dokumentation?  → SOFORT Wiki erstellen (Blocker)
□ Ticket ohne Umsetzung?               → Status: TODO
□ Umsetzung ohne Ticket?               → SOFORT Issue erstellen
□ Roadmap-Eintrag ohne Epic?           → Epic ableiten
□ Epic ohne Roadmap-Eintrag?           → Roadmap aktualisieren
```

---

## WIKI-VALIDIERUNG

Wiki-Status-Lifecycle:
```
DRAFT → REVIEW → APPROVED → PUBLISHED → DEPRECATED
```

Vor PUBLISHED: Alle 5 Checks müssen grün sein:
- [ ] Technisch korrekt (Code stimmt überein)
- [ ] Konsistent mit Code (Klassen/Methoden exakt)
- [ ] Konsistent mit Architektur (Layer stimmt)
- [ ] Konsistent mit Standards (ATC-Nummer vorhanden)
- [ ] Konsistent mit Roadmap (Sprint stimmt)

---

## ISSUE-ERSTELLUNG

**Pflichtfelder für jedes neue Issue:**

```yaml
title: "[Kap. XX] Komponente — Kurzbeschreibung"
description: |
  ## Was
  ## Warum
  ## Akzeptanzkriterien
  - [ ] Kriterium 1 (messbar)
  - [ ] Kriterium 2 (messbar)
  - [ ] Kriterium 3 (messbar)
priority: CRITICAL | HIGH | MEDIUM | LOW
sprint: X.X
wiki_chapter: Kap. XX
standard: ATC-XXXX
depends_on: #XX, #XX
```

**Nur erstellen wenn:**
- GitHub API ✅ erreichbar
- Kein Duplikat ✅ (Titel nicht bereits vorhanden)

---

## RELEASE-BLOCKER

Release ist GESPERRT solange:

```
❌ Audit-Score < 80/100
❌ Kritische Sicherheitslücke offen
❌ Neue Klasse/API ohne Wiki-Kapitel
❌ Neues Modul ohne Tests
❌ Offene CRITICAL/HIGH Blocker-Issues
❌ Standard im DRAFT-Status (nicht APPROVED)
❌ Offene DECISION-Items die den Release betreffen
❌ CI/CD schlägt fehl
❌ Wiki-Kapitel im DRAFT-Status
```

---

## SYNC-REIHENFOLGE (nach jeder Änderung)

```
1. GitHub Code → ✅/❌ mit Commit-SHA
2. GitHub Wiki → ✅/❌ mit Commit-SHA
3. EcosystemNode (Base44) → Zustand aktualisieren
4. Wiki-Status → PUBLISHED setzen
5. Issues → Verlinkungen prüfen/ergänzen
6. Roadmap / Sprint → Status aktualisieren
7. CHANGELOG.md → Eintrag hinzufügen
8. Notion → Tagesprotokoll
9. Google Tasks → Offene Tasks synchronisieren
10. Google Drive → Report hochladen
11. Gmail + Outlook → Status-Report senden
```

Jeden Schritt mit Status ✅/❌ protokollieren.

---

## OFFENE DECISIONS (MENSCH ERFORDERLICH)

Diese Items werden bei **jedem** Sync-Lauf gemeldet bis sie resolved sind:

| ID | Impact | Thema | Deadline |
|----|--------|-------|---------|
| AD-004 | 🔴 CRITICAL | Chain-ID 9000 vs. XDC Network | 20.06.2026 |
| AD-006 | 🔴 CRITICAL | Python vs. Substrate Runtime | 30.06.2026 |
| AD-001 | 🟡 HIGH | SHA-256 vs. Keccak-256 | Sprint 3.0 |
| AD-003 | 🟡 HIGH | Voting-Power Snapshot | vor Mainnet |
| AD-002 | 🟠 MEDIUM | EventBus vs. IPCBus | Sprint 2.4 |
| AD-005 | 🟠 MEDIUM | AIP-001 Spezifikation | Sprint 2.3 |
| AD-007 | 🔴 CRITICAL | EVM Chain Registry Konflikt | 20.06.2026 |

---

*Diese Policy ist verbindlich für Aurora (A-Town Ecosystem Brain)*
*Automatisch durchgesetzt bei jedem Sync-Lauf (täglich 08:00 Uhr)*
*Letzte Aktualisierung: 2026-06-12*
