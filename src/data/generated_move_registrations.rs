// Auto-generated move registrations for MoveFactory
// Generated from legacy MOVES HashMap

use crate::choices::{Choices, Flags, Secondary, Effect, Boost, StatBoosts, VolatileStatus, Heal, Status, SideCondition};
use crate::data::move_service::EngineDataBuilder;
use crate::data::types::MoveTarget;
use crate::state::{PokemonStatus, PokemonType, PokemonSideCondition};
use crate::engine::state::PokemonVolatileStatus;
use super::move_factory::MoveFactory;

impl MoveFactory {
    /// Register all engine-specific move data
    pub async fn register_all_engine_data(&self) {

        // Draining moves
        self.service.register_engine_data(
            Choices::PARABOLICCHARGE,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MEGADRAIN,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DREAMEATER,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GIGADRAIN,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRAINPUNCH,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LEECHLIFE,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OBLIVIONWING,
            EngineDataBuilder::new()
                .drain(0.75)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ABSORB,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MATCHAGOTCHA,
            EngineDataBuilder::new()
                .drain(0.5)
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BOUNCYBUBBLE,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HORNLEECH,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRAININGKISS,
            EngineDataBuilder::new()
                .drain(0.75)
                .flags(Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BITTERBLADE,
            EngineDataBuilder::new()
                .drain(0.5)
                .flags(Flags {
                    contact: true,
                    heal: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                })
                .build()
        ).await;

        // Recoil moves
        self.service.register_engine_data(
            Choices::VOLTTACKLE,
            EngineDataBuilder::new()
                .recoil(0.33)
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WILDCHARGE,
            EngineDataBuilder::new()
                .recoil(0.25)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WOODHAMMER,
            EngineDataBuilder::new()
                .recoil(0.33)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEADSMASH,
            EngineDataBuilder::new()
                .recoil(0.5)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLAREBLITZ,
            EngineDataBuilder::new()
                .recoil(0.33)
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WAVECRASH,
            EngineDataBuilder::new()
                .recoil(0.33)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TAKEDOWN,
            EngineDataBuilder::new()
                .recoil(0.33)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DOUBLEEDGE,
            EngineDataBuilder::new()
                .recoil(0.33)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BRAVEBIRD,
            EngineDataBuilder::new()
                .recoil(0.33)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEADCHARGE,
            EngineDataBuilder::new()
                .recoil(0.25)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LIGHTOFRUIN,
            EngineDataBuilder::new()
                .recoil(0.5)
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SUBMISSION,
            EngineDataBuilder::new()
                .recoil(0.25)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;

        // Stat-boosting moves
        self.service.register_engine_data(
            Choices::HYPERSPACEFURY,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TICKLE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ACIDARMOR,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MEMENTO,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                })
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -2,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MAKEITRAIN,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHELLSMASH,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: -1,
                        special_attack: 2,
                        special_defense: -1,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BABYDOLLEYES,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GROWL,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TEARFULLOOK,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AMNESIA,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 2,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::METALSOUND,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FEATHERDANCE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRAGONASCENT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TOXICTHREAD,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                })
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::POISON,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MINIMIZE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MINIMIZE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLATTER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SCARYFACE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -2,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROCKPOLISH,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::VCREATE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: -1,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TIDYUP,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HOWL,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::COTTONGUARD,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 3,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CHARM,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHARPEN,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLASH,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SCALESHOT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BULKUP,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SMOKESCREEN,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LEER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NASTYPLOT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SANDATTACK,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRAGONDANCE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DEFENSECURL,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::DEFENSECURL,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SCREECH,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PLAYNICE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::COSMICPOWER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NORETREAT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::NORETREAT,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HARDEN,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HONECLAWS,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 1,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LEAFSTORM,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BARRIER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPICYEXTRACT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: -2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TAILWHIP,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHIFTGEAR,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GROWTH,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NOBLEROAR,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WITHDRAW,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLEURCANNON,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TARSHOT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::TARSHOT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SWAGGER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEADLONGRUSH,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TAILGLOW,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 3,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MEDITATE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CLOSECOMBAT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: -1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CONFIDE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DOUBLETEAM,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::EERIEIMPULSE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::VICTORYDANCE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 1,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::QUIVERDANCE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 1,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICEHAMMER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HAMMERARM,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PARTINGSHOT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: 0,
                        special_attack: -1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    pivot: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CHARGE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::CHARGE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CURSE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -1,
                        accuracy: 0,
                    },
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::CURSE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WORKUP,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GEOMANCY,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 2,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    charge: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::IRONDEFENSE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PSYCHOBOOST,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OVERHEAT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::COACHING,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::COIL,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 1,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 1,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SWEETSCENT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SUPERPOWER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: -1,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STRINGSHOT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -2,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DEFENDORDER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 1,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AGILITY,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DECORATE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CLANGINGSCALES,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: -1,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHELTER,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 2,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FAKETEARS,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: -2,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CALMMIND,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 1,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DEFOG,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AUTOTOMIZE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 2,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::COTTONSPORE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -2,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SWORDSDANCE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 2,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AROMATICMIST,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 1,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CAPTIVATE,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::KINESIS,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::SelectedPokemon,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: 0,
                        accuracy: -1,
                    },
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPINOUT,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: 0,
                        special_defense: 0,
                        speed: -2,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRACOMETEOR,
            EngineDataBuilder::new()
                .boost(Boost {
                    target: MoveTarget::User,
                    boosts: StatBoosts {
                        attack: 0,
                        defense: 0,
                        special_attack: -2,
                        special_defense: 0,
                        speed: 0,
                        accuracy: 0,
                    },
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;

        // Status-inflicting moves
        self.service.register_engine_data(
            Choices::AQUARING,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::AQUARING,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::IMPRISON,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::IMPRISON,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NIGHTMARE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::NIGHTMARE,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CONFUSERAY,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WHIRLPOOL,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HYPNOSIS,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POISONGAS,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::POISON,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPIKYSHIELD,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SPIKYSHIELD,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIRESPIN,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OBSTRUCT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SILKTRAP,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SILKTRAP,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::KINGSSHIELD,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::KINGSSHIELD,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MAGNETRISE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MAGNETRISE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDERCAGE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CLAMP,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SING,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::RAGINGFURY,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MAGMASTORM,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::EMBARGO,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::EMBARGO,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POWERTRICK,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::POWERTRICK,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BURNINGBULWARK,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BURNINGBULWARK,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::RAGEPOWDER,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::RAGEPOWDER,
                })
                .flags(Flags {
                    powder: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WRAP,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TOXIC,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::TOXIC,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPORE,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POISONPOWDER,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::POISON,
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THRASH,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ENDURE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::ENDURE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ODORSLEUTH,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::FORESIGHT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PROTECT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TAUNT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::TAUNT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BIDE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BIDE,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DESTINYBOND,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::DESTINYBOND,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPOTLIGHT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::SPOTLIGHT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::INGRAIN,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::INGRAIN,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SALTCURE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::SALTCURE,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MAGICCOAT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::MAGICCOAT,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LEECHSEED,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::LEECHSEED,
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SMACKDOWN,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::SMACKDOWN,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GASTROACID,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::GASTROACID,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WILLOWISP,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::BURN,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HELPINGHAND,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::HELPINGHAND,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SANDTOMB,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THROATCHOP,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::THROATCHOP,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TELEKINESIS,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::TELEKINESIS,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SLEEPPOWDER,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OCTOLOCK,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::OCTOLOCK,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OUTRAGE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDERWAVE,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::PARALYZE,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PETALDANCE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LOCKEDMOVE,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STOCKPILE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::STOCKPILE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DARKVOID,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROOST,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::ROOST,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SUBSTITUTE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SUBSTITUTE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TORMENT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::TORMENT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FORESIGHT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::FORESIGHT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BIND,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GRUDGE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::GRUDGE,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DETECT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::PROTECT,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FOCUSENERGY,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::FOCUSENERGY,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SWEETKISS,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STUNSPORE,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::PARALYZE,
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::YAWN,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::YAWN,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ENCORE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::ENCORE,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PSYCHICNOISE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::HEALBLOCK,
                })
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LOVELYKISS,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::INFESTATION,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THOUSANDARROWS,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::SMACKDOWN,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GLARE,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::PARALYZE,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LASERFOCUS,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::LASERFOCUS,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POWERSHIFT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::POWERSHIFT,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MIRACLEEYE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::MIRACLEEYE,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BANEFULBUNKER,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::BANEFULBUNKER,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ELECTRIFY,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::ELECTRIFY,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEALBLOCK,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::HEALBLOCK,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FOLLOWME,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::FOLLOWME,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SNAPTRAP,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::PARTIALLYTRAPPED,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ATTRACT,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::ATTRACT,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POWDER,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::POWDER,
                })
                .flags(Flags {
                    powder: true,
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SUPERSONIC,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GRASSWHISTLE,
            EngineDataBuilder::new()
                .status(Status {
                    target: MoveTarget::SelectedPokemon,
                    status: PokemonStatus::SLEEP,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DISABLE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::DISABLE,
                })
                .flags(Flags {
                    protect: true,
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TEETERDANCE,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::SelectedPokemon,
                    volatile_status: PokemonVolatileStatus::CONFUSION,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SNATCH,
            EngineDataBuilder::new()
                .volatile_status(VolatileStatus {
                    target: MoveTarget::User,
                    volatile_status: PokemonVolatileStatus::SNATCH,
                })
                .build()
        ).await;

        // Moves with secondary effects
        self.service.register_engine_data(
            Choices::EMBER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SKITTERSMACK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DARKPULSE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    pulse: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICEPUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::IRONTAIL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ASTONISH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OCTAZOOKA,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::INFERNALPARADE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLOATYFALL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POISONTAIL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BODYSLAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BONECLUB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FAKEOUT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SIGNALBEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MYSTICALPOWER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AURORABEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GLACIATE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WICKEDTORQUE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::SLEEP)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIREBLAST,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::INFERNO,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPLISHYSPLASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICEBEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NIGHTDAZE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 40.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICEBURN,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DIZZYPUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WATERFALL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICYWIND,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SKYATTACK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SANDSEARSTORM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MYSTICALFIRE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MIRRORSHOT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CONFUSION,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIREFANG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LOWSWEEP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ACID,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BLIZZARD,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TRAILBLAZE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIERYDANCE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CONSTRICT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROLLINGKICK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIRELASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TROPKICK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NEEDLEARM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PYROBALL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SNORE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BARBBARRAGE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHADOWSTRIKE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BLAZEKICK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BUBBLEBEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ELECTROWEB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ESPERWING,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::RAZORSHELL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPRINGTIDESTORM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BUGBUZZ,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::METALCLAW,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LIQUIDATION,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::RELICSONG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::SLEEP)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DYNAMICPUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TRIATTACK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 6.67,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                    Secondary {
                        chance: 6.67,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                    Secondary {
                        chance: 6.67,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POISONFANG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::TOXIC)
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STRUGGLEBUG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LICK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICICLECRASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::METEORMASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POWERUPPUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::EARTHPOWER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BOUNCE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    charge: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDERPUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CRUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SYRUPBOMB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::SYRUPBOMB)
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BLAZINGTORQUE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TWISTER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STEELWING,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CHARGEBEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 70.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FREEZINGGLARE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FORCEPALM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NUZZLE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HYPERFANG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHADOWBONE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MUDBOMB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SEEDFLARE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 40.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -2,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GRAVAPPLE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::UPPERHAND,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SIZZLYSLIDE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CHILLINGWATER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AQUASTEP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POUNCE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LUNGE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BUZZYBUZZ,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FREEZEDRY,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LUSTERPURGE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SMOG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 40.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDERBOLT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROCKTOMB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SECRETPOWER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MISTBALL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLASHCANNON,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BITE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ICEFANG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HURRICANE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDEROUSKICK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHELLSIDEARM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DOUBLEIRONBASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STRANGESTEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CHATTER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MAGICALTORQUE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PLAYROUGH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDERSHOCK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::RAPIDSPIN,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRAGONBREATH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FICKLEBEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SNARL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLAMETHROWER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STEAMROLLER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LEAFTORNADO,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SACREDFIRE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POWDERSNOW,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::FREEZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STOMP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::IRONHEAD,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIREPUNCH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    punch: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BREAKINGSWIPE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROCKSMASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LAVAPLUME,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CROSSPOISON,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::GUNKSHOT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROCKSLIDE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLAMEWHEEL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TRIPLEARROWS,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BITTERMALICE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::APPLEACID,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ANCIENTPOWER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 1,
                            special_attack: 1,
                            special_defense: 1,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEADBUTT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BULLDOZE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MORTALSPIN,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::NOXIOUSTORQUE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CRUSHCLAW,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: -1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ZIPPYZAP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MUDSHOT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEATWAVE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WATERPULSE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    pulse: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SLUDGEWAVE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHADOWBALL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PSYSHIELDBASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 1,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TWINEEDLE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SILVERWIND,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 1,
                            special_attack: 1,
                            special_defense: 1,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FREEZESHOCK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    charge: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ENERGYBALL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPARK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FLAMECHARGE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRUMBEATING,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DISCHARGE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PALEOWAVE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: -1,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MALIGNANTCHAIN,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::TOXIC)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DRAGONRUSH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPARKLINGARIA,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::SPARKLINGARIA)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BLEAKWINDSTORM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PSYCHIC,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POISONSTING,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MOONBLAST,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ZAPCANNON,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AURAWHEEL,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STEAMERUPTION,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ZENHEADBUTT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SCALD,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AXEKICK,
            EngineDataBuilder::new()
                .crash(0.5)
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LUMINACRASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -2,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEARTSTAMP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SLUDGEBOMB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BLUEFLARE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SCORCHINGSANDS,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AIRSLASH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    slicing: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::DIAMONDSTORM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 50.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 2,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MUDDYWATER,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPIRITBREAK,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: -1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SLUDGE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ROCKCLIMB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::EXTRASENSORY,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ZINGZAP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MOUNTAINGALE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FIERYWRATH,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MUDSLAP,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: 0,
                            accuracy: -1,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::POISONJAB,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::POISON)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::OMINOUSWIND,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 1,
                            defense: 1,
                            special_attack: 1,
                            special_defense: 1,
                            speed: 1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::ACIDSPRAY,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -2,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SEARINGSHOT,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::BURN)
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TORCHSONG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 100.0,
                        target: MoveTarget::User,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 1,
                            special_defense: 0,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    sound: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WILDBOLTSTORM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BUBBLE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: 0,
                            speed: -1,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::FOCUSBLAST,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Boost(StatBoosts {
                            attack: 0,
                            defense: 0,
                            special_attack: 0,
                            special_defense: -1,
                            speed: 0,
                            accuracy: 0,
                        })
,
                    },
                ])
                .flags(Flags {
                    bullet: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::PSYBEAM,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::CONFUSION)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::COMBATTORQUE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 30.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::BOLTSTRIKE,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 20.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                ])
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::THUNDERFANG,
            EngineDataBuilder::new()
                .secondaries(vec![
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::Status(PokemonStatus::PARALYZE)
,
                    },
                    Secondary {
                        chance: 10.0,
                        target: MoveTarget::SelectedPokemon,
                        effect: Effect::VolatileStatus(PokemonVolatileStatus::FLINCH)
,
                    },
                ])
                .flags(Flags {
                    bite: true,
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;

        // Other special moves
        self.service.register_engine_data(
            Choices::SOFTBOILED,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STEALTHROCK,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::SelectedPokemon,
                    condition: PokemonSideCondition::Stealthrock,
                })
                .flags(Flags {
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SYNTHESIS,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SUPERCELLSLAM,
            EngineDataBuilder::new()
                .crash(0.5)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::WIDEGUARD,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::WideGuard,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STEELBEAM,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: -0.5,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::JUMPKICK,
            EngineDataBuilder::new()
                .crash(0.5)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SPIKES,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::SelectedPokemon,
                    condition: PokemonSideCondition::Spikes,
                })
                .flags(Flags {
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SAFEGUARD,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Safeguard,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEALORDER,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::JUNGLEHEALING,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.25,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LIGHTSCREEN,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::LightScreen,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::AURORAVEIL,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::AuroraVeil,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MILKDRINK,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HIGHJUMPKICK,
            EngineDataBuilder::new()
                .crash(0.5)
                .flags(Flags {
                    contact: true,
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STICKYWEB,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::SelectedPokemon,
                    condition: PokemonSideCondition::StickyWeb,
                })
                .flags(Flags {
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::QUICKGUARD,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::QuickGuard,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LIFEDEW,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.25,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MORNINGSUN,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CEASELESSEDGE,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::SelectedPokemon,
                    condition: PokemonSideCondition::Spikes,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TAILWIND,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Tailwind,
                })
                .flags(Flags {
                    wind: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LUCKYCHANT,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::LuckyChant,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::HEALINGWISH,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: -1.0,
                })
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::HealingWish,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::STONEAXE,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::SelectedPokemon,
                    condition: PokemonSideCondition::Stealthrock,
                })
                .flags(Flags {
                    contact: true,
                    protect: true,
                    slicing: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::TOXICSPIKES,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::SelectedPokemon,
                    condition: PokemonSideCondition::ToxicSpikes,
                })
                .flags(Flags {
                    reflectable: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SLACKOFF,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::REFLECT,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Reflect,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MATBLOCK,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::MatBlock,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::RECOVER,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::SHOREUP,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CRAFTYSHIELD,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::CraftyShield,
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::CHLOROBLAST,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: -0.5,
                })
                .flags(Flags {
                    protect: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MOONLIGHT,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.5,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::LUNARBLESSING,
            EngineDataBuilder::new()
                .heal(Heal {
                    target: MoveTarget::User,
                    amount: 0.25,
                })
                .flags(Flags {
                    heal: true,
                    ..Default::default()
                })
                .build()
        ).await;
        self.service.register_engine_data(
            Choices::MIST,
            EngineDataBuilder::new()
                .side_condition(SideCondition {
                    target: MoveTarget::User,
                    condition: PokemonSideCondition::Mist,
                })
                .build()
        ).await;
    }
}
