# inkplate-osrs

a rust server to track character state from runelite in real-time, and an Inkplate 10 Arduino sketch which queries the rust server.

we source runelite events using the existing `rseye-connector` plugin. we resolve item IDs using the `osrsreboxed-db` JSON definitions.

## setup

### osrsreboxed-db hack

we rely on the `osrsreboxed-db` item ID JSON files, so you need to clone the repository (until i integrate it better):

```
; git clone https://github.com/0xNeffarion/osrsreboxed-db --depth 1
Cloning into 'osrsreboxed-db'...
remote: Enumerating objects: 45539, done.
remote: Counting objects: 100% (45539/45539), done.
remote: Compressing objects: 100% (41767/41767), done.
remote: Total 45539 (delta 4530), reused 36500 (delta 3769), pack-reused 0 (from 0)
Receiving objects: 100% (45539/45539), 71.49 MiB | 15.82 MiB/s, done.
Resolving deltas: 100% (4530/4530), done.
Updating files: 100% (60621/60621), done.
```

### server + inkplate display

- install [`rseye-connector`](https://runelite.net/plugin-hub/show/rseye-connector) from Runelite Plugin Hub
- set "Endpoint Configuration" appropriately (`http://localhost/`)
- set the `OSRS_USERNAME` environment variable
- start the rust server (it uses port 80 by default)
- configure `Inkplate_OSRS_Display/config.h` with your WiFi SSID + Password and rust server URL
- flash the inkplate with the `arduino-cli`

## flashing the inkplate

replace with the virtual file representing your serial device:

``` bash
arduino-cli compile \
  --upload \
  --fqbn Inkplate_Boards:esp32:Inkplate10:UploadSpeed=115200 \
  -p /dev/cu.usbserial-214430 ./Inkplate_OSRS_Display
```

## running the server

```bash
OSRS_USERNAME='purple djan' cargo run
```

```
[Running 'cargo run']
   Compiling inkplate-osrs v0.1.0 (/Users/jstrickland/code/inkplate-osrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.06s
     Running `target/debug/inkplate-osrs`
2025-12-01T23:23:51.868928Z  INFO inkplate_osrs: starting!
2025-12-01T23:23:51.869211Z  INFO inkplate_osrs: Loading item database from osrsreboxed-db...
2025-12-01T23:23:52.988416Z  INFO inkplate_osrs: Loaded 28744 items
2025-12-01T23:23:53.022035Z  INFO inkplate_osrs: Fetching baseline stats for purple djan
2025-12-01T23:23:54.409604Z  INFO inkplate_osrs: Successfully fetched baseline stats for purple djan
```

As you perform actions ingame, you should see corresponding logs:

```

2025-12-01T23:27:50.334725Z  INFO inkplate_osrs: Received update: PositionUpdate {
    username: "purple djan",
    position: WorldPoint {
        x: 3058,
        y: 3310,
        plane: 0,
    },
}
2025-12-01T23:28:11.376531Z  INFO inkplate_osrs: Received update: StatUpdate {
    username: "purple djan",
    combat_level: 113,
    stat_changes: [
        StatChange {
            boosted_level: 84,
            level: 84,
            skill: "FARMING",
            xp: 3055159,
        },
    ],
}
2025-12-01T23:28:13.134843Z  INFO inkplate_osrs: Received update: StatUpdate {
    username: "purple djan",
    combat_level: 113,
    stat_changes: [
        StatChange {
            boosted_level: 84,
            level: 84,
            skill: "FARMING",
            xp: 3055193,
        },
    ],
}
```

## querying `/status`

### json schema

```bash
; curl http://localhost/status 2>/dev/null | jq 'keys'
```
```json
[
  "bank",
  "equipment",
  "inventory",
  "lastDeathTime",
  "lastLoot",
  "loginState",
  "overhead",
  "position",
  "questPoints",
  "quests",
  "questsCompleted",
  "skull",
  "stats",
  "totalQuests",
  "username"
]
```

### stats

```bash
; curl http://localhost/status 2>/dev/null | jq '.stats'
```

<details> <summary>Response JSON</summary>

```json
{
  "username": "purple djan",
  "combatLevel": 113,
  "statChanges": [
    {
      "boostedLevel": 91,
      "level": 91,
      "skill": "ATTACK",
      "xp": 6219905
    },
    {
      "boostedLevel": 90,
      "level": 90,
      "skill": "DEFENCE",
      "xp": 5434067
    },
    {
      "boostedLevel": 90,
      "level": 90,
      "skill": "STRENGTH",
      "xp": 5427295
    },
    {
      "boostedLevel": 93,
      "level": 93,
      "skill": "HITPOINTS",
      "xp": 7629793
    },
    {
      "boostedLevel": 89,
      "level": 89,
      "skill": "RANGED",
      "xp": 4937923
    },
    {
      "boostedLevel": 69,
      "level": 69,
      "skill": "PRAYER",
      "xp": 677211
    },
    {
      "boostedLevel": 82,
      "level": 82,
      "skill": "MAGIC",
      "xp": 2643150
    },
    {
      "boostedLevel": 75,
      "level": 75,
      "skill": "COOKING",
      "xp": 1283566
    },
    {
      "boostedLevel": 70,
      "level": 70,
      "skill": "WOODCUTTING",
      "xp": 763527
    },
    {
      "boostedLevel": 64,
      "level": 64,
      "skill": "FLETCHING",
      "xp": 430985
    },
    {
      "boostedLevel": 64,
      "level": 64,
      "skill": "FISHING",
      "xp": 411720
    },
    {
      "boostedLevel": 80,
      "level": 80,
      "skill": "FIREMAKING",
      "xp": 2020981
    },
    {
      "boostedLevel": 63,
      "level": 63,
      "skill": "CRAFTING",
      "xp": 399749
    },
    {
      "boostedLevel": 71,
      "level": 71,
      "skill": "SMITHING",
      "xp": 832112
    },
    {
      "boostedLevel": 70,
      "level": 70,
      "skill": "MINING",
      "xp": 800216
    },
    {
      "boostedLevel": 63,
      "level": 63,
      "skill": "HERBLORE",
      "xp": 395385
    },
    {
      "boostedLevel": 68,
      "level": 68,
      "skill": "AGILITY",
      "xp": 660503
    },
    {
      "boostedLevel": 63,
      "level": 63,
      "skill": "THIEVING",
      "xp": 388082
    },
    {
      "boostedLevel": 75,
      "level": 75,
      "skill": "SLAYER",
      "xp": 1262486
    },
    {
      "boostedLevel": 84,
      "level": 84,
      "skill": "FARMING",
      "xp": 3055525
    },
    {
      "boostedLevel": 60,
      "level": 60,
      "skill": "RUNECRAFTING",
      "xp": 275768
    },
    {
      "boostedLevel": 63,
      "level": 63,
      "skill": "HUNTER",
      "xp": 396546
    },
    {
      "boostedLevel": 70,
      "level": 70,
      "skill": "CONSTRUCTION",
      "xp": 758328
    }
  ]
}
```

</details>

### equipment
```bash
; curl http://localhost/status 2>/dev/null | jq '.equipment'
```

<details>
<summary>Response JSON</summary>

```json
{
  "AMULET": {
    "id": 11970,
    "quantity": 1,
    "name": "Skills necklace(5)"
  },
  "WEAPON": {
    "id": 7409,
    "quantity": 1,
    "name": "Magic secateurs"
  },
  "LEGS": {
    "id": 11857,
    "quantity": 1,
    "name": "Graceful legs"
  },
  "BOOTS": {
    "id": 11861,
    "quantity": 1,
    "name": "Graceful boots"
  },
  "GLOVES": {
    "id": 11859,
    "quantity": 1,
    "name": "Graceful gloves"
  },
  "SHIELD": {
    "id": 25818,
    "quantity": 1,
    "name": "Book of the dead"
  },
  "BODY": {
    "id": 11855,
    "quantity": 1,
    "name": "Graceful top"
  },
  "CAPE": {
    "id": 11853,
    "quantity": 1,
    "name": "Graceful cape"
  },
  "HEAD": {
    "id": 11851,
    "quantity": 1,
    "name": "Graceful hood"
  },
  "RING": {
    "id": 13126,
    "quantity": 1,
    "name": "Explorer's ring 2"
  }
}

```

</details>

### inventory

``` bash
; curl localhost/status 2>/dev/null | jq '.inventory'
```
<details>
<summary>Response JSON</summary>

```json
[
  {
    "id": 12791,
    "quantity": 1,
    "name": "Rune pouch"
  },
  {
    "id": 563,
    "quantity": 230,
    "name": "Law rune"
  },
  {
    "id": 5295,
    "quantity": 2,
    "name": "Ranarr seed"
  },
  {
    "id": 22997,
    "quantity": 1,
    "name": "Bottomless compost bucket"
  },
  {
    "id": 24478,
    "quantity": 1,
    "name": "Open herb sack"
  },
  {
    "id": 5341,
    "quantity": 1,
    "name": "Rake"
  },
  {
    "id": 5343,
    "quantity": 1,
    "name": "Seed dibber"
  },
  {
    "id": 952,
    "quantity": 1,
    "name": "Spade"
  },
  {
    "id": 24961,
    "quantity": 32,
    "name": "Catherby teleport"
  },
  {
    "id": 13122,
    "quantity": 1,
    "name": "Ardougne cloak 2"
  },
  {
    "id": 4251,
    "quantity": 1,
    "name": "Ectophial"
  },
  {
    "id": 12625,
    "quantity": 1,
    "name": "Stamina potion(4)"
  },
  {
    "id": 995,
    "quantity": 5240,
    "name": "Coins"
  },
  {
    "id": 22875,
    "quantity": 1,
    "name": "Hespori seed"
  },
  {
    "id": 258,
    "quantity": 34,
    "name": "Ranarr weed"
  },
  {
    "id": 6055,
    "quantity": 1,
    "name": "Weeds"
  }
]

```
</details>

### quest info

``` bash
; curl localhost/status 2>/dev/null | jq '. | { questsCompleted, totalQuests, questPoints }'
```

```json
{
  "questsCompleted": 176,
  "totalQuests": 211,
  "questPoints": 282
}
```
