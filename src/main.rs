use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    id: i32,
    quantity: i32,
}

#[derive(Debug, Clone, Serialize)]
struct ItemWithName {
    id: i32,
    quantity: i32,
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatChange {
    boosted_level: u32,
    level: u32,
    skill: String,
    xp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Quest {
    id: u32,
    name: String,
    state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EquipmentUpdate {
    username: String,
    items: HashMap<String, Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InventoryUpdate {
    username: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BankUpdate {
    username: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatUpdate {
    username: String,
    combat_level: u32,
    stat_changes: Vec<StatChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestUpdate {
    username: String,
    quest_points: u32,
    quest_changes: Vec<Quest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WorldPoint {
    x: i32,
    y: i32,
    plane: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PositionUpdate {
    username: String,
    position: WorldPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginUpdate {
    username: String,
    state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LootUpdate {
    username: String,
    loot_type: Option<String>,
    entity_id: Option<i32>,
    entity_name: Option<String>,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LootUpdateWithNames {
    username: String,
    loot_type: Option<String>,
    entity_id: Option<i32>,
    entity_name: Option<String>,
    items: Vec<ItemWithName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeathUpdate {
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverheadUpdate {
    username: String,
    overhead: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SkullUpdate {
    username: String,
    skull: i32,
}

#[derive(Debug, Default, Clone)]
struct PlayerState {
    username: Option<String>,
    position: Option<WorldPoint>,
    login_state: Option<String>,
    equipment: Option<HashMap<String, Item>>,
    inventory: Option<Vec<Item>>,
    bank: Option<Vec<Item>>,
    stats: Option<StatUpdate>,
    quests: Option<QuestUpdate>,
    quests_completed: Option<u32>,
    total_quests: Option<u32>,
    quest_points: Option<u32>,
    last_loot: Option<LootUpdate>,
    last_death_time: Option<String>,
    overhead: Option<String>,
    skull: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PlayerStateWithNames {
    username: Option<String>,
    position: Option<WorldPoint>,
    login_state: Option<String>,
    equipment: Option<HashMap<String, ItemWithName>>,
    inventory: Option<Vec<ItemWithName>>,
    bank: Option<Vec<ItemWithName>>,
    stats: Option<StatUpdate>,
    quests: Option<QuestUpdate>,
    quests_completed: Option<u32>,
    total_quests: Option<u32>,
    quest_points: Option<u32>,
    last_loot: Option<LootUpdateWithNames>,
    last_death_time: Option<String>,
    overhead: Option<String>,
    skull: Option<i32>,
}

trait UpdateState {
    fn update_state(&self, state: &mut PlayerState);
}

impl UpdateState for EquipmentUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.equipment = Some(self.items.clone());
    }
}

impl UpdateState for InventoryUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.inventory = Some(self.items.clone());
    }
}

impl UpdateState for BankUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.bank = Some(self.items.clone());
    }
}

impl UpdateState for StatUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());

        // Merge stat changes with existing stats instead of replacing
        if let Some(existing_stats) = &mut state.stats {
            // Update combat level
            existing_stats.combat_level = self.combat_level;

            // Merge stat changes - update existing skills or add new ones
            for new_stat in &self.stat_changes {
                if let Some(existing_stat) = existing_stats
                    .stat_changes
                    .iter_mut()
                    .find(|s| s.skill == new_stat.skill)
                {
                    // Update existing skill
                    *existing_stat = new_stat.clone();
                } else {
                    // Add new skill
                    existing_stats.stat_changes.push(new_stat.clone());
                }
            }
        } else {
            // No existing stats, just set it
            state.stats = Some(self.clone());
        }
    }
}

impl UpdateState for QuestUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.quests = Some(self.clone());
        state.quest_points = Some(self.quest_points);
        state.total_quests = Some(self.quest_changes.len() as u32);

        // Count completed quests
        let completed = self
            .quest_changes
            .iter()
            .filter(|q| q.state == "FINISHED")
            .count() as u32;
        state.quests_completed = Some(completed);
    }
}

impl UpdateState for PositionUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.position = Some(self.position.clone());
    }
}

impl UpdateState for LoginUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.login_state = Some(self.state.clone());
    }
}

impl UpdateState for LootUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.last_loot = Some(self.clone());
    }
}

impl UpdateState for DeathUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.last_death_time = Some(chrono::Utc::now().to_rfc3339());
    }
}

impl UpdateState for OverheadUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.overhead = self.overhead.clone();
    }
}

impl UpdateState for SkullUpdate {
    fn update_state(&self, state: &mut PlayerState) {
        state.username = Some(self.username.clone());
        state.skull = Some(self.skull);
    }
}

#[derive(Clone)]
struct AppState {
    player_state: Arc<RwLock<PlayerState>>,
    item_db: Arc<HashMap<i32, String>>,
}

fn load_item_database() -> Result<HashMap<i32, String>, Box<dyn std::error::Error>> {
    info!("Loading item database from osrsreboxed-db...");

    let json_path = "osrsreboxed-db/docs/items-complete.json";
    let contents = fs::read_to_string(json_path)?;

    let items: serde_json::Value = serde_json::from_str(&contents)?;

    let mut item_map = HashMap::new();

    if let Some(items_obj) = items.as_object() {
        for (id_str, item_data) in items_obj {
            if let Ok(id) = id_str.parse::<i32>() {
                if let Some(name) = item_data.get("name").and_then(|n| n.as_str()) {
                    item_map.insert(id, name.to_string());
                }
            }
        }
    }

    info!("Loaded {} items", item_map.len());
    Ok(item_map)
}

async fn fetch_baseline_stats(username: &str) -> Result<PlayerState, Box<dyn std::error::Error>> {
    info!("Fetching baseline stats for {}", username);

    let player = osrs_highscores::standard_high_scores(username)?;

    let mut state = PlayerState::default();
    state.username = Some(player.name.clone());

    // Create a StatUpdate from highscores data
    let mut stat_changes = Vec::new();

    for stat in &player.stats {
        // Skip "Overall" stat (first one)
        if stat.skill == "Overall" {
            continue;
        }

        // Convert skill name to uppercase to match RuneLite format
        let skill_name = stat.skill.to_uppercase();

        stat_changes.push(StatChange {
            boosted_level: stat.level as u32,
            level: stat.level as u32,
            skill: skill_name,
            xp: stat.xp as u32,
        });
    }

    // Calculate combat level from stats
    let combat_level = calculate_combat_level(&stat_changes);

    state.stats = Some(StatUpdate {
        username: player.name,
        combat_level,
        stat_changes,
    });

    Ok(state)
}

fn calculate_combat_level(stats: &[StatChange]) -> u32 {
    let get_level = |skill: &str| -> f64 {
        stats
            .iter()
            .find(|s| s.skill == skill)
            .map(|s| s.level as f64)
            .unwrap_or(1.0)
    };

    let attack = get_level("ATTACK");
    let defence = get_level("DEFENCE");
    let strength = get_level("STRENGTH");
    let hitpoints = get_level("HITPOINTS");
    let prayer = get_level("PRAYER");
    let ranged = get_level("RANGED");
    let magic = get_level("MAGIC");

    let base = 0.25 * (defence + hitpoints + (prayer / 2.0).floor());
    let melee = 0.325 * (attack + strength);
    let range = 0.325 * ((ranged * 3.0) / 2.0).floor();
    let mage = 0.325 * ((magic * 3.0) / 2.0).floor();

    let combat = base + melee.max(range).max(mage);
    combat.floor() as u32
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("starting!");

    // Load item database
    let item_db = match load_item_database() {
        Ok(db) => Arc::new(db),
        Err(e) => {
            info!(
                "Failed to load item database: {}. Item IDs will not be resolved.",
                e
            );
            Arc::new(HashMap::new())
        }
    };

    // Get username from environment variable (required)
    let username =
        env::var("OSRS_USERNAME").expect("OSRS_USERNAME environment variable must be set");

    // Fetch baseline stats from highscores
    let initial_state = match fetch_baseline_stats(&username).await {
        Ok(state) => {
            info!("Successfully fetched baseline stats for {}", username);
            state
        }
        Err(e) => {
            info!(
                "Failed to fetch baseline stats: {}. Starting with empty state.",
                e
            );
            PlayerState::default()
        }
    };

    let app_state = AppState {
        player_state: Arc::new(RwLock::new(initial_state)),
        item_db,
    };

    let app = Router::new()
        .route("/position_update/", post(api::<PositionUpdate>))
        .route("/login_update/", post(api::<LoginUpdate>))
        .route("/stat_update/", post(api::<StatUpdate>))
        .route("/quest_update/", post(api::<QuestUpdate>))
        .route("/bank_update/", post(api::<BankUpdate>))
        .route("/loot_update/", post(api::<LootUpdate>))
        .route("/inventory_update/", post(api::<InventoryUpdate>))
        .route("/equipment_update/", post(api::<EquipmentUpdate>))
        .route("/death_update/", post(api::<DeathUpdate>))
        .route("/overhead_update/", post(api::<OverheadUpdate>))
        .route("/skull_update/", post(api::<SkullUpdate>))
        .route("/status", get(get_status))
        .route("/{*wildcard}", post(fallback_api))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:80").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn api<T>(State(app_state): State<AppState>, Json(payload): Json<T>) -> impl IntoResponse
where
    T: std::fmt::Debug + serde::de::DeserializeOwned + UpdateState,
{
    info!("Received update: {:#?}", payload);

    let mut player_state = app_state.player_state.write().await;
    payload.update_state(&mut player_state);

    StatusCode::OK
}

async fn get_status(State(app_state): State<AppState>) -> impl IntoResponse {
    let player_state = app_state.player_state.read().await;

    // Helper function to resolve item names
    let resolve_item = |item: &Item| -> ItemWithName {
        ItemWithName {
            id: item.id,
            quantity: item.quantity,
            name: app_state.item_db.get(&item.id).cloned(),
        }
    };

    // Convert PlayerState to PlayerStateWithNames by resolving all item IDs
    let state_with_names = PlayerStateWithNames {
        username: player_state.username.clone(),
        position: player_state.position.clone(),
        login_state: player_state.login_state.clone(),
        equipment: player_state.equipment.as_ref().map(|eq| {
            eq.iter()
                .map(|(slot, item)| (slot.clone(), resolve_item(item)))
                .collect()
        }),
        inventory: player_state
            .inventory
            .as_ref()
            .map(|inv| inv.iter().map(resolve_item).collect()),
        bank: player_state
            .bank
            .as_ref()
            .map(|bank| bank.iter().map(resolve_item).collect()),
        stats: player_state.stats.clone(),
        quests: player_state.quests.clone(),
        quests_completed: player_state.quests_completed,
        total_quests: player_state.total_quests,
        quest_points: player_state.quest_points,
        last_loot: player_state
            .last_loot
            .as_ref()
            .map(|loot| LootUpdateWithNames {
                username: loot.username.clone(),
                loot_type: loot.loot_type.clone(),
                entity_id: loot.entity_id,
                entity_name: loot.entity_name.clone(),
                items: loot.items.iter().map(resolve_item).collect(),
            }),
        last_death_time: player_state.last_death_time.clone(),
        overhead: player_state.overhead.clone(),
        skull: player_state.skull,
    };

    Json(state_with_names)
}

async fn fallback_api(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    info!("Received unknown update type: {:#?}", payload);
    StatusCode::OK
}
