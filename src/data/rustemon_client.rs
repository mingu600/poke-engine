use rustemon::client::RustemonClient;
use rustemon::model::pokemon::{Pokemon, PokemonSpecies};
use rustemon::model::moves::Move;
use rustemon::model::items::Item;
use std::sync::Arc;

/// Wrapper around RustemonClient for poke-engine data access
pub struct PokeEngineDataClient {
    client: Arc<RustemonClient>,
}

impl PokeEngineDataClient {
    /// Create a new client with default configuration
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = RustemonClient::default();
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Fetch a Pokemon by name
    pub async fn get_pokemon(&self, name: &str) -> Result<Pokemon, Box<dyn std::error::Error>> {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(name, &self.client).await?;
        Ok(pokemon)
    }

    /// Fetch a Pokemon by ID
    pub async fn get_pokemon_by_id(&self, id: i64) -> Result<Pokemon, Box<dyn std::error::Error>> {
        let pokemon = rustemon::pokemon::pokemon::get_by_id(id, &self.client).await?;
        Ok(pokemon)
    }

    /// Fetch a Pokemon species by name
    pub async fn get_pokemon_species(&self, name: &str) -> Result<PokemonSpecies, Box<dyn std::error::Error>> {
        let species = rustemon::pokemon::pokemon_species::get_by_name(name, &self.client).await?;
        Ok(species)
    }

    /// Fetch a move by name
    pub async fn get_move(&self, name: &str) -> Result<Move, Box<dyn std::error::Error>> {
        let move_data = rustemon::moves::move_::get_by_name(name, &self.client).await?;
        Ok(move_data)
    }

    /// Fetch a move by ID
    pub async fn get_move_by_id(&self, id: i64) -> Result<Move, Box<dyn std::error::Error>> {
        let move_data = rustemon::moves::move_::get_by_id(id, &self.client).await?;
        Ok(move_data)
    }

    /// Fetch an item by name
    pub async fn get_item(&self, name: &str) -> Result<Item, Box<dyn std::error::Error>> {
        let item = rustemon::items::item::get_by_name(name, &self.client).await?;
        Ok(item)
    }

    /// Fetch all Pokemon (paginated access)
    pub async fn get_all_pokemon(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Pokemon>, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);
        
        let pokemon_list = rustemon::pokemon::pokemon::get_page_with_param(offset, limit, &self.client).await?;
        let mut pokemon_data = Vec::new();
        
        for pokemon_ref in pokemon_list.results {
            let pokemon_name = &pokemon_ref.name;
            match self.get_pokemon(pokemon_name).await {
                Ok(pokemon) => pokemon_data.push(pokemon),
                Err(e) => eprintln!("Failed to fetch Pokemon {}: {}", pokemon_name, e),
            }
        }
        
        Ok(pokemon_data)
    }

    /// Fetch all moves (paginated access)
    pub async fn get_all_moves(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Move>, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);
        
        let moves_list = rustemon::moves::move_::get_page_with_param(offset, limit, &self.client).await?;
        let mut moves_data = Vec::new();
        
        for move_ref in moves_list.results {
            let move_name = &move_ref.name;
            match self.get_move(move_name).await {
                Ok(move_data) => moves_data.push(move_data),
                Err(e) => eprintln!("Failed to fetch move {}: {}", move_name, e),
            }
        }
        
        Ok(moves_data)
    }

    /// Fetch all items (paginated access)
    pub async fn get_all_items(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);
        
        let items_list = rustemon::items::item::get_page_with_param(offset, limit, &self.client).await?;
        let mut items_data = Vec::new();
        
        for item_ref in items_list.results {
            let item_name = &item_ref.name;
            match self.get_item(item_name).await {
                Ok(item) => items_data.push(item),
                Err(e) => eprintln!("Failed to fetch item {}: {}", item_name, e),
            }
        }
        
        Ok(items_data)
    }

    /// Helper to get English name from a name array
    pub fn get_english_name(names: &[rustemon::model::resource::Name]) -> Option<String> {
        names.iter()
            .find(|name| {
                name.language.name == "en"
            })
            .map(|name| name.name.clone())
    }

    /// Helper to normalize names for engine compatibility (similar to battle_environment.rs)
    pub fn normalize_name(name: &str) -> String {
        name.replace(" ", "")
            .replace("-", "")
            .replace(".", "")
            .replace("'", "")
            .replace("%", "")
            .replace("*", "")
            .replace(":", "")
            .replace("(", "")
            .replace(")", "")
            .trim()
            .to_lowercase()
    }
}

impl Default for PokeEngineDataClient {
    fn default() -> Self {
        Self::new().expect("Failed to create PokeEngineDataClient")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_pokemon() {
        let client = PokeEngineDataClient::new().unwrap();
        let pikachu = client.get_pokemon("pikachu").await;
        assert!(pikachu.is_ok());
        
        let pokemon = pikachu.unwrap();
        assert_eq!(pokemon.name, "pikachu".to_string());
        assert!(pokemon.stats.len() > 0);
    }

    #[tokio::test]
    async fn test_fetch_move() {
        let client = PokeEngineDataClient::new().unwrap();
        let thunderbolt = client.get_move("thunderbolt").await;
        assert!(thunderbolt.is_ok());
        
        let move_data = thunderbolt.unwrap();
        assert_eq!(move_data.name, "thunderbolt".to_string());
    }

    #[tokio::test]
    async fn test_name_normalization() {
        assert_eq!(PokeEngineDataClient::normalize_name("Mr. Mime"), "mrmime");
        assert_eq!(PokeEngineDataClient::normalize_name("Type: Null"), "typenull");
        assert_eq!(PokeEngineDataClient::normalize_name("Tapu Koko"), "tapukoko");
    }
}