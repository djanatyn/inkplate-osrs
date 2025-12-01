# OSRS Player Status Display for Inkplate10

E-ink display showing real-time OSRS player stats, skills, and inventory from the RuneLite connector.

## Features

- **Combat Level** - Prominently displayed
- **All 24 Skills** - Shows current and boosted levels
- **Position** - Current world coordinates
- **Quest Progress** - Quest points earned
- **Inventory** - Item count and total quantities
- **Timestamps** - Last updated and next update times in local time

## Hardware Required

- Inkplate 10 (1200x825 e-ink display)
- WiFi network access

## Setup

1. Copy `config.h.example` to `config.h`:
   ```bash
   cp config.h.example config.h
   ```

2. Edit `config.h` with your credentials:
   - WiFi SSID and password
   - Server host (default: 192.168.1.210)
   - Timezone offset

3. Upload to Inkplate10 using Arduino IDE or arduino-cli:
   ```bash
   arduino-cli compile --fqbn esp32:esp32:inkplate10 Inkplate_OSRS_Display
   arduino-cli upload -p /dev/ttyUSB0 --fqbn esp32:esp32:inkplate10 Inkplate_OSRS_Display
   ```

## Display Layout

```
┌─────────────────────────────────────────────────────────┐
│ username                              CB 113             │
│ Position: (3163, 3487, 0)                               │
├─────────────────────────────────────────────────────────┤
│ ATTACK: 91    RANGED: 89     MINING: 70   RC: 60       │
│ DEFENCE: 90   PRAYER: 54/69  HERB: 63     HUNTER: 63   │
│ STRENGTH: 90  MAGIC: 82      AGILITY: 68  CON: 70      │
│ HP: 93        COOKING: 75    THIEV: 63    SAILING: 46  │
│ ...           ...             ...          ...          │
├─────────────────────────────────────────────────────────┤
│ Quests: 282 QP                                          │
│ Inventory: 12 items (145 total qty)                    │
│                                                          │
│ Last Updated: 2025-12-01 02:30:15 PM                   │
│ Next Update: 2025-12-01 02:31:15 PM                    │
└─────────────────────────────────────────────────────────┘
```

## Update Behavior

- Refreshes every 60 seconds
- Shows absolute timestamps (not countdowns) so display remains accurate when unplugged
- Reconnects to WiFi automatically if connection drops

## Dependencies

- Inkplate library
- ArduinoJson (v7+)
- ESP32 WiFi/HTTPClient (included with ESP32 core)
