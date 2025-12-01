/*
   OSRS Player Status Display for Inkplate10

   Displays RuneScape player stats, skills, and inventory
   from the OSRS tracking server.

   Setup:
   1. Copy config.h.example to config.h
   2. Edit config.h with your WiFi credentials and server address
   3. Upload to Inkplate10
*/

#include "Inkplate.h"
#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#include <time.h>
#include "config.h"

// Update interval
const unsigned long UPDATE_INTERVAL = 60000; // 60 seconds

// Create Inkplate object in 1-bit mode (black and white)
Inkplate display(INKPLATE_1BIT);

// Timing
unsigned long lastUpdate = 0;
time_t nextUpdateTime = 0;

// Player data structure
struct PlayerData {
    String username;
    int combatLevel;
    int questPoints;
    int questsCompleted;
    int totalQuests;
    int posX, posY, posPlane;

    // Skills (24 total)
    struct Skill {
        String name;
        int level;
        int boostedLevel;
    };
    Skill skills[24];
    int skillCount;

    // Inventory
    int inventorySlotsUsed;
};

PlayerData playerData;
bool dataValid = false;
time_t lastUpdateTime = 0;

void setup() {
    Serial.begin(115200);
    Serial.println("OSRS Display Starting...");

    // Initialize display
    display.begin();
    display.clearDisplay();
    display.setTextColor(BLACK, WHITE);
    display.setTextSize(1);

    // Show connecting message
    display.setTextSize(2);
    display.setCursor(400, 400);
    display.print("Connecting to WiFi...");
    display.display();

    // Connect to WiFi
    connectWiFi();

    // Initialize time
    configTime(TIMEZONE_OFFSET, 0, "pool.ntp.org", "time.nist.gov");

    // Wait for time to be set
    Serial.print("Waiting for NTP time sync");
    time_t now = time(nullptr);
    int attempts = 0;
    while (now < 1000000000 && attempts < 20) {
        delay(500);
        Serial.print(".");
        now = time(nullptr);
        attempts++;
    }
    Serial.println();

    // Initial data fetch
    fetchAndDisplay();
}

void loop() {
    // Check WiFi connection
    if (WiFi.status() != WL_CONNECTED) {
        Serial.println("WiFi disconnected, reconnecting...");
        connectWiFi();
    }

    // Periodic update
    if (millis() - lastUpdate >= UPDATE_INTERVAL) {
        fetchAndDisplay();
    }

    delay(1000);
}

void connectWiFi() {
    Serial.print("Connecting to WiFi: ");
    Serial.println(WIFI_SSID);

    WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

    int attempts = 0;
    while (WiFi.status() != WL_CONNECTED && attempts < 30) {
        delay(500);
        Serial.print(".");
        attempts++;
    }

    if (WiFi.status() == WL_CONNECTED) {
        Serial.println("\nConnected!");
        Serial.print("IP: ");
        Serial.println(WiFi.localIP());
    } else {
        Serial.println("\nFailed to connect to WiFi");
    }
}

void fetchAndDisplay() {
    lastUpdate = millis();

    Serial.println("Fetching player data...");

    if (fetchPlayerData()) {
        Serial.println("Data fetched successfully, updating display...");
        displayPlayerData();
        lastUpdateTime = time(nullptr);
        nextUpdateTime = lastUpdateTime + (UPDATE_INTERVAL / 1000);
    } else {
        Serial.println("Failed to fetch data");
        displayError();
    }
}

bool fetchPlayerData() {
    if (WiFi.status() != WL_CONNECTED) {
        return false;
    }

    HTTPClient http;
    String url = String("http://") + SERVER_HOST + ":" + SERVER_PORT + "/status";

    Serial.print("Requesting: ");
    Serial.println(url);

    http.begin(url);
    int httpCode = http.GET();

    if (httpCode != 200) {
        Serial.printf("HTTP request failed: %d\n", httpCode);
        http.end();
        return false;
    }

    String payload = http.getString();
    http.end();

    // Parse JSON
    JsonDocument doc;
    DeserializationError error = deserializeJson(doc, payload);

    if (error) {
        Serial.print("JSON parsing failed: ");
        Serial.println(error.c_str());
        return false;
    }

    // Extract data
    playerData.username = doc["username"].as<String>();

    // Position
    if (doc["position"].is<JsonObject>()) {
        playerData.posX = doc["position"]["x"];
        playerData.posY = doc["position"]["y"];
        playerData.posPlane = doc["position"]["plane"];
    }

    // Stats
    if (doc["stats"].is<JsonObject>()) {
        playerData.combatLevel = doc["stats"]["combatLevel"];

        JsonArray statChanges = doc["stats"]["statChanges"];
        playerData.skillCount = 0;

        for (JsonObject stat : statChanges) {
            if (playerData.skillCount < 24) {
                playerData.skills[playerData.skillCount].name = stat["skill"].as<String>();
                playerData.skills[playerData.skillCount].level = stat["level"];
                playerData.skills[playerData.skillCount].boostedLevel = stat["boostedLevel"];
                playerData.skillCount++;
            }
        }
    }

    // Quest data from top-level fields (calculated server-side)
    playerData.questPoints = doc["questPoints"].as<int>();
    playerData.questsCompleted = doc["questsCompleted"].as<int>();
    playerData.totalQuests = doc["totalQuests"].as<int>();

    // Inventory - count non-empty slots
    if (doc["inventory"].is<JsonArray>()) {
        JsonArray inventory = doc["inventory"];
        playerData.inventorySlotsUsed = 0;

        for (JsonObject item : inventory) {
            int id = item["id"];
            int quantity = item["quantity"];
            if (id != -1 && quantity > 0) {
                playerData.inventorySlotsUsed++;
            }
        }
    }

    dataValid = true;
    return true;
}

void displayPlayerData() {
    display.clearDisplay();

    // Header: Username, Position, Combat Level
    display.setTextSize(3);
    display.setCursor(10, 10);
    display.print(playerData.username);

    display.setCursor(10, 40);
    display.setTextSize(2);
    display.print("Position: (");
    display.print(playerData.posX);
    display.print(", ");
    display.print(playerData.posY);
    display.print(", ");
    display.print(playerData.posPlane);
    display.print(")");

    // Combat level - large and prominent
    display.setTextSize(3);
    display.setCursor(700, 10);
    display.print("Combat Level: ");
    display.print(playerData.combatLevel);

    // Divider line
    display.drawLine(0, 70, 1200, 70, BLACK);

    // Skills section - display in 4 columns x 6 rows
    int startY = 90;
    int colWidth = 290;
    int rowHeight = 100;

    for (int i = 0; i < playerData.skillCount && i < 24; i++) {
        int col = i / 6;  // 6 rows per column
        int row = i % 6;

        int x = 10 + (col * colWidth);
        int y = startY + (row * rowHeight);

        // Skill name on top line (size 2)
        display.setTextSize(2);
        display.setCursor(x, y);
        display.print(playerData.skills[i].name);

        // Level on bottom line (size 3, larger and indented)
        display.setTextSize(3);
        display.setCursor(x + 20, y + 35);

        // Show boosted level if different from base level
        if (playerData.skills[i].boostedLevel != playerData.skills[i].level) {
            display.print(playerData.skills[i].boostedLevel);
            display.print("/");
        }
        display.print(playerData.skills[i].level);
    }

    // Bottom section - Quest count and Inventory
    int bottomY = startY + (6 * rowHeight) + 20;
    display.drawLine(0, bottomY - 10, 1200, bottomY - 10, BLACK);

    display.setTextSize(2);
    display.setCursor(10, bottomY);
    display.print("Quests: ");
    display.print(playerData.questsCompleted);
    display.print("/");
    display.print(playerData.totalQuests);
    display.print(" completed (");
    display.print(playerData.questPoints);
    display.print(" QP)");

    display.setCursor(10, bottomY + 25);
    display.print("Inventory: ");
    display.print(playerData.inventorySlotsUsed);
    display.print("/28 slots used");

    // Timestamps
    display.setCursor(10, bottomY + 60);
    display.print("Last Updated: ");
    display.print(formatTime(lastUpdateTime));

    display.setCursor(10, bottomY + 85);
    display.print("Next Update: ");
    display.print(formatTime(nextUpdateTime));

    display.display();
}

void displayError() {
    display.clearDisplay();
    display.setTextSize(2);
    display.setCursor(300, 400);
    display.print("Failed to fetch data");

    display.setTextSize(1);
    display.setCursor(300, 430);
    display.print("Will retry in 60 seconds...");

    display.display();
}

String formatTime(time_t t) {
    if (t == 0) {
        return "Unknown";
    }

    struct tm* timeinfo = localtime(&t);
    char buffer[32];
    strftime(buffer, sizeof(buffer), "%Y-%m-%d %I:%M:%S %p", timeinfo);
    return String(buffer);
}
