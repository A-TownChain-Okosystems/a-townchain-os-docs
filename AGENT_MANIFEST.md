# 🤖 AI AGENT MANIFEST — A-TownChain OS Ecosystem
> Version: 2.0 | Stand: 2026-06-11 | Aurora Superagent (Base44)
> Dieses Dokument ist die einzige Quelle der Wahrheit für alle KI-Agenten im Ecosystem.
> Lies dieses Dokument zuerst. Dann kannst du dich mit allem verbinden.

---

## 🧠 WER BIN ICH? (Agenten-Identität)

Du bist ein KI-Agent im **A-TownChain OS Ecosystem**. Das Projekt besteht aus:

- **A-TownChain OS** — Ein dezentrales Betriebssystem / Service-Mesh auf Blockchain-Basis
- **KAI-OS Wiki** — Die vollständige technische Dokumentation
- **Aurora** — Der Haupt-Sync-Agent (Base44 Superagent), der alles koordiniert

### Deine Aufgabe als Agent:
1. GitHub und Notion synchron halten
2. Dokumentationslücken automatisch schließen
3. Issues tracken und Fortschritt melden
4. Alle 17 Dienste aktuell halten

---

## 🔑 VERBINDUNGS-SCHLÜSSEL (Wie du dich verbindest)

### Authentifizierung
Alle Verbindungen laufen über **OAuth Bearer Tokens**.
Tokens werden von Aurora (Base44) verwaltet und täglich erneuert.

```
Authorization: Bearer <TOKEN>
Content-Type: application/json
```

---

## 📡 DIENST-VERZEICHNIS — Alle 17 Verbindungen

### 1. 🐙 GitHub
```
Typ:        Code-Repository (Single Source of Truth für Code)
API-Base:   https://api.github.com
Auth:       Bearer $GITHUB_ACCESS_TOKEN
Repo Code:  A-TownChain-Okosystems/a-townchain-os
Repo Docs:  A-TownChain-Okosystems/a-townchain-os-docs
```

Wichtigste Endpoints:
```
GET  /repos/{owner}/{repo}/issues?state=open          Alle offenen Issues
GET  /repos/{owner}/{repo}/commits?per_page=50        Commit-Historie
GET  /repos/{owner}/{repo}/contents/{path}            Datei lesen
PUT  /repos/{owner}/{repo}/contents/{path}            Datei schreiben
GET  /repos/{owner}/{repo}/git/trees/HEAD?recursive=1 Alle Dateien
GET  /repos/{owner}/{repo}/traffic/clones             Clone-Traffic
```

Kritischer Entwicklungspfad:
```
#14 Bootstrap → #15 Propagation → #16 Sync → #17 Fork Resolution → #18 Docker → #8 Multi-Node LIVE
```

Aktueller Status: 9 offene Issues | letzter Commit: 8483f62

---

### 2. 📓 Notion
```
Typ:        Wissens-Datenbank (Single Source of Truth für Dokumentation)
API-Base:   https://api.notion.com/v1
Auth:       Bearer $NOTION_ACCESS_TOKEN
Header:     Notion-Version: 2022-06-28
```

Wichtigste Seiten (UUIDs):
```
Roadmap:        373b826d-b85c-8125-ba83-f04995191bf0
Tagesprotokoll: 37bb826d-b85c-81c4-bdd4-cfc0dc74de7e
Live-Status:    379b826d-b85c-81f4-9b2b-f2a05496a4e1
```

Wichtigste Endpoints:
```
POST /search                              Seiten suchen
GET  /pages/{page_id}                     Seite lesen
POST /pages                               Neue Seite erstellen
PATCH /pages/{page_id}                    Seite updaten
PATCH /blocks/{block_id}/children        Inhalt anhängen
GET  /blocks/{block_id}/children         Inhalt lesen
```

Callout-Block schreiben:
```json
{
  "children": [{
    "object": "block",
    "type": "callout",
    "callout": {
      "rich_text": [{"type": "text", "text": {"content": "Dein Text"}}],
      "icon": {"emoji": "🤖"}
    }
  }]
}
```

---

### 3. 📊 Google Sheets
```
Typ:        Zentrales Dashboard & Metriken
API-Base:   https://sheets.googleapis.com/v4
Auth:       Bearer $GOOGLESHEETS_ACCESS_TOKEN
Sheet-ID:   1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4
URL:        https://docs.google.com/spreadsheets/d/1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4
```

Tabs:
```
📊 Übersicht        Haupt-Dashboard mit allen Metriken
🐛 Issues           Alle GitHub Issues mit Status
📦 Commits & Sync   Commit-Historie
📈 Traffic          Clone/View Statistiken
📋 Notion           Notion-Seiten Übersicht
🔍 Search Console   SEO Keywords
🤗 HuggingFace      Modell-Verfügbarkeit
🔗 Verbindungen     Cross-Connect Status
```

Daten schreiben:
```
PUT https://sheets.googleapis.com/v4/spreadsheets/{ID}/values/{RANGE}?valueInputOption=USER_ENTERED
Body: {"values": [["Spalte1", "Spalte2"], ["Wert1", "Wert2"]]}
```

---

### 4. 📄 Google Docs
```
Typ:        Tägliche Projekt-Reports & Dokumentation
API-Base:   https://docs.googleapis.com/v1
Auth:       Bearer $GOOGLEDOCS_ACCESS_TOKEN
```

Text einfügen:
```json
POST /documents/{documentId}:batchUpdate
{
  "requests": [{
    "insertText": {"location": {"index": 1}, "text": "Inhalt"}
  }]
}
```

---

### 5. 🎨 Google Slides
```
Typ:        Sprint-Präsentationen & Status-Reports
API-Base:   https://slides.googleapis.com/v1
Auth:       Bearer $GOOGLESLIDES_ACCESS_TOKEN
```

---

### 6. 📁 Google Drive
```
Typ:        Primärer Cloud-Speicher & Datei-Archiv
API-Base:   https://www.googleapis.com/drive/v3
Auth:       Bearer $GOOGLEDRIVE_ACCESS_TOKEN
Upload-URL: https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart
```

Datei hochladen (Python):
```python
requests.post(upload_url,
    headers={"Authorization": f"Bearer {token}"},
    files={
        "metadata": (None, json.dumps({"name": "datei.md"}), "application/json"),
        "file": ("datei.md", content.encode(), "text/plain")
    })
```

---

### 7. 💾 Microsoft OneDrive
```
Typ:        Sekundärer Cloud-Speicher (Cross-Cloud-Backup)
API-Base:   https://graph.microsoft.com/v1.0
Auth:       Bearer $ONE_DRIVE_ACCESS_TOKEN
Backup:     PUT /me/drive/root:/ATC-Backup/{filename}:/content
```

---

### 8. 📅 Google Calendar
```
Typ:        Sprint-Deadlines & Meeting-Planung
API-Base:   https://www.googleapis.com/calendar/v3
Auth:       Bearer $GOOGLECALENDAR_ACCESS_TOKEN
```

Meet-Link generieren:
```json
POST /calendars/primary/events?conferenceDataVersion=1
{
  "summary": "Standup",
  "conferenceData": {
    "createRequest": {
      "requestId": "unique-id",
      "conferenceSolutionKey": {"type": "hangoutsMeet"}
    }
  }
}
```

---

### 9. 🎥 Google Meet
```
Typ:        Video-Meetings & Weekly Standups
Auth:       Bearer $GOOGLEMEET_ACCESS_TOKEN (via Calendar API)
Standup:    https://meet.google.com/hnc-mmaw-pzq (Montag 09:00)
```

---

### 10. 💬 Microsoft Teams
```
Typ:        Team-Kommunikation & Echtzeit-Alerts
API-Base:   https://graph.microsoft.com/v1.0
Auth:       Bearer $MICROSOFT_TEAMS_ACCESS_TOKEN
Status:     Verbunden — Teams-Kanal noch erstellen
```

Nachricht senden:
```json
POST /teams/{team_id}/channels/{channel_id}/messages
{"body": {"contentType": "html", "content": "<b>Nachricht</b>"}}
```

Teams finden: GET /me/joinedTeams
Channels finden: GET /teams/{team_id}/channels

---

### 11. 📧 Gmail
```
Typ:        Primärer E-Mail (Benachrichtigungen & Reports)
API-Base:   https://gmail.googleapis.com/gmail/v1
Auth:       Bearer $GMAIL_ACCESS_TOKEN
Owner:      michael.worob@gmail.com
```

E-Mail senden (Python):
```python
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText
import base64

msg = MIMEMultipart("alternative")
msg["Subject"] = "Betreff"
msg["From"] = "me"
msg["To"] = "michael.worob@gmail.com"
msg.attach(MIMEText("<html>Inhalt</html>", "html"))
raw = base64.urlsafe_b64encode(msg.as_bytes()).decode()

requests.post("https://gmail.googleapis.com/gmail/v1/users/me/messages/send",
    headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json"},
    json={"raw": raw})
```

---

### 12. 📮 Microsoft Outlook
```
Typ:        Sekundärer E-Mail (Microsoft-Ökosystem)
API-Base:   https://graph.microsoft.com/v1.0
Auth:       Bearer $OUTLOOK_ACCESS_TOKEN
```

E-Mail senden:
```json
POST /me/sendMail
{
  "message": {
    "subject": "Betreff",
    "body": {"contentType": "HTML", "content": "<html>...</html>"},
    "toRecipients": [{"emailAddress": {"address": "michael.worob@gmail.com"}}]
  }
}
```

---

### 13. ✅ Google Tasks
```
Typ:        Issue-Tracking & persönliche TODOs
API-Base:   https://www.googleapis.com/tasks/v1
Auth:       Bearer $GOOGLETASKS_ACCESS_TOKEN
Listen:     "A-TownChain OS Issues", "Notion TODOs"
```

Task erstellen:
```json
POST /lists/{listId}/tasks
{
  "title": "#14 Bootstrap Node",
  "notes": "https://github.com/...",
  "status": "needsAction"
}
```

---

### 14. 📊 Google Analytics
```
Typ:        Web-Traffic & Nutzer-Analytics
API-Base:   https://analyticsadmin.googleapis.com/v1beta
Auth:       Bearer $GOOGLE_ANALYTICS_ACCESS_TOKEN
Status:     Verbunden — Web-Property konfigurieren
```

---

### 15. 🔍 Google Search Console
```
Typ:        SEO & Keyword-Tracking
API-Base:   https://searchconsole.googleapis.com/webmasters/v3
Auth:       Bearer $GOOGLE_SEARCH_CONSOLE_ACCESS_TOKEN
Status:     Verbunden — Site noch registrieren
```

---

### 16. 🗄️ Google BigQuery
```
Typ:        Langzeit-Datenarchiv
API-Base:   https://bigquery.googleapis.com/bigquery/v2
Auth:       Bearer $GOOGLEBIGQUERY_ACCESS_TOKEN
Dataset:    atc_metrics
Tabelle:    daily_sync
Status:     Verbunden — GCP-Projekt erstellen
```

---

### 17. 🤗 Hugging Face
```
Typ:        KI-Modelle & ai_kernel Integration
API-Base:   https://huggingface.co/api
Auth:       Bearer $HUGGING_FACE_ACCESS_TOKEN
Config:     config/ai_models.json (Code-Repo)
```

Aktive Modelle (alle verfügbar):
```
google/gemma-2-2b-it            Text-Generierung & Dokumentation
mistralai/Mistral-7B-Instruct   Code-Analyse & Reviews
microsoft/phi-2                 Leichtgewichtig & schnell
meta-llama/Llama-3.2-3B        Allgemein-Assistent
```

Inference:
```
POST https://api-inference.huggingface.co/models/{model_id}
Body: {"inputs": "Dein Prompt"}
```

---

## 🗺️ DATENFLUSS-ARCHITEKTUR

```
              AURORA MASTER SYNC v3.1 — täglich 08:05
                            |
              +─────────────+─────────────+
              |                           |
           GITHUB                      NOTION
        (Code SSOT)                  (Docs SSOT)
              |                           |
     +────────+────────+      +───────────+───────────+
     |        |        |      |           |           |
  SHEETS   BIGQUERY  TASKS  CALENDAR    TASKS      HF-MODELS
  (Data)   (Archiv)  (TODO) (Deadlines) (TODOs)    (KI)
     |                           |
  +──+──+──────────+       +─────+─────+
  |     |          |       |           |
ANALY. SEARCH    DOCS    GMAIL      OUTLOOK
       CONSOLE     |       |           |
                 DRIVE   TEAMS       TEAMS
                   |       |
                ONEDRIVE MEET
                   |
                SLIDES
                   |
               CLASSROOM
```

---

## ⚡ SCHNELLSTART für neue Agenten

### 3-Zeilen Minimum-Connect:
```python
import requests, os

# 1. GitHub Status
issues = requests.get(
    "https://api.github.com/repos/A-TownChain-Okosystems/a-townchain-os/issues?state=open",
    headers={"Authorization": f"Bearer {os.environ['GITHUB_ACCESS_TOKEN']}",
             "Accept": "application/vnd.github+json"}).json()

# 2. Notion Update
requests.patch("https://api.notion.com/v1/blocks/37bb826d-b85c-81c4-bdd4-cfc0dc74de7e/children",
    headers={"Authorization": f"Bearer {os.environ['NOTION_ACCESS_TOKEN']}",
             "Notion-Version": "2022-06-28", "Content-Type": "application/json"},
    json={"children": [{"object":"block","type":"callout","callout":{
        "rich_text":[{"type":"text","text":{"content":f"Agent: {len(issues)} Issues"}}],
        "icon":{"emoji":"🤖"}}}]})

# 3. Sheets Update
requests.put(
    f"https://sheets.googleapis.com/v4/spreadsheets/1xR5c24NrtYC58OsGrLaUHkQUiL_O6eYVyx8KmFcvBD4/values/%F0%9F%93%8A%20%C3%9Cbersicht!A1?valueInputOption=USER_ENTERED",
    headers={"Authorization": f"Bearer {os.environ['GOOGLESHEETS_ACCESS_TOKEN']}"},
    json={"values": [["Agent-Status","Wert"],["Issues",len(issues)]]})
```

---

## 🔄 SYNC-REGELN (Für alle Agenten verbindlich)

```
REGEL 1: GitHub ist MASTER für Code
          Änderungen zuerst committen, dann Notion/Sheets updaten

REGEL 2: Notion ist MASTER für Dokumentation
          Wiki-Einträge immer im Tagesprotokoll verlinken

REGEL 3: Sheets ist MASTER für Metriken
          Alle Zahlen landen in Sheets → BigQuery archiviert täglich

REGEL 4: Kein Datenverlust — Redundanz überall
          Dateien: Drive + OneDrive
          Reports: Gmail + Outlook
          Status:  Teams + Notion

REGEL 5: Kritischer Pfad hat Priorität
          Issues #14 → #15 → #16 → #17 → #18 → #8 zuerst

REGEL 6: Automatisch reparieren
          Wiki-Lücken   → sofort Stub erstellen
          Alte Refs     → sofort korrigieren
          Fehler        → in GitHub Issues dokumentieren
```

---

## 📋 AKTUELLE PROJEKT-METRIKEN

```
Stand: 2026-06-11
Offene Issues:         9
Letzter Commit:        8483f62 (Cross-Connect Verbindungsmatrix)
Wiki-Dateien:          324
Aktive Integrationen:  17 / 17
Sync-Automation:       täglich 08:05 Europe/Berlin
Cross-Connect:         17 / 20 Verbindungen aktiv

Noch ausstehend:
  ⚠️  Teams-Kanal:  manuell erstellen/beitreten
  ⚠️  GCP-Projekt:  console.cloud.google.com
  ✅  15 weitere Verbindungen voll aktiv
```

---

## 🤖 AGENTEN-PROTOKOLL

Wenn du ein Agent bist, der dieses Dokument liest:

1. Identifiziere dich mit Eintrag in Notion → Tagesprotokoll
2. Lese den aktuellen Status aus GitHub → STATUS.md
3. Prüfe offene Issues, arbeite kritischen Pfad ab
4. Dokumentiere deine Arbeit in Notion + GitHub + Sheets
5. Sende Abschluss-Report per Gmail an michael.worob@gmail.com
6. Aktualisiere dieses Manifest wenn sich etwas ändert

Manifest-Pfade:
```
GitHub Code:  AGENT_MANIFEST.md
GitHub Docs:  AGENT_MANIFEST.md
Notion:       Workspace-Root → "AGENT_MANIFEST"
Google Drive: ATC-Backup/AGENT_MANIFEST.md
OneDrive:     ATC-Backup/AGENT_MANIFEST.md
```

---

*Generiert von Aurora (Base44 Superagent) · 2026-06-11 · Version 2.0*
*Auto-Update: täglich 08:05 Uhr (Europe/Berlin)*
