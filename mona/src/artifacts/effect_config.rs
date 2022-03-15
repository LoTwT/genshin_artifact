use serde::{Serialize, Deserialize};

use crate::common::Element;
use crate::common::max_trait::MaxValue;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ConfigArchaicPetra {
    pub element: Element,
    pub rate: f64,
}

impl Default for ConfigArchaicPetra {
    fn default() -> Self {
        ConfigArchaicPetra {
            element: Element::Pyro,
            rate: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ConfigBlizzardStrayer {
    pub critical_bonus: f64,
}

impl Default for ConfigBlizzardStrayer {
    fn default() -> Self {
        ConfigBlizzardStrayer {
            critical_bonus: 0.0
        }
    }
}

impl MaxValue for ConfigBlizzardStrayer {
    fn max_value() -> Self {
        ConfigBlizzardStrayer {
            critical_bonus: 0.4
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ConfigPaleFlame {
    pub avg_level: f64,
    pub full_rate: f64,
}

impl Default for ConfigPaleFlame {
    fn default() -> Self {
        ConfigPaleFlame {
            avg_level: 0.0,
            full_rate: 0.0,
        }
    }
}

impl MaxValue for ConfigPaleFlame {
    fn max_value() -> Self {
        ConfigPaleFlame {
            avg_level: 2.0,
            full_rate: 1.0
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ConfigRate {
    pub rate: f64,
}

impl MaxValue for ConfigRate {
    fn max_value() -> Self {
        ConfigRate {
            rate: 1.0
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ConfigLevel {
    pub level: f64,
}

impl Default for ConfigRate {
    fn default() -> Self {
        ConfigRate {
            rate: 0.0,
        }
    }
}

impl Default for ConfigLevel {
    fn default() -> Self {
        ConfigLevel {
            level: 0.0,
        }
    }
}

#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ArtifactEffectConfig {
    pub config_archaic_petra: ConfigArchaicPetra,
    pub config_berserker: ConfigRate,
    pub config_blizzard_strayer: ConfigBlizzardStrayer,
    pub config_bloodstained_chivalry: ConfigRate,
    pub config_brave_heart: ConfigRate,
    pub config_crimson_witch_of_flames: ConfigLevel,
    pub config_heart_of_depth: ConfigRate,
    pub config_husk_of_opulent_dreams: ConfigLevel,
    pub config_instructor: ConfigRate,
    pub config_lavawalker: ConfigRate,
    pub config_martial_artist: ConfigRate,
    pub config_noblesse_oblige: ConfigRate,
    pub config_pale_flame: ConfigPaleFlame,
    pub config_retracing_bolide: ConfigRate,
    pub config_shimenawas_reminiscence: ConfigRate,
    pub config_tenacity_of_the_millelith: ConfigRate,
    pub config_thundersoother: ConfigRate,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ArtifactConfigInterface {
    pub config_archaic_petra: Option<ConfigArchaicPetra>,
    pub config_berserker: Option<ConfigRate>,
    pub config_blizzard_strayer: Option<ConfigBlizzardStrayer>,
    pub config_bloodstained_chivalry: Option<ConfigRate>,
    pub config_brave_heart: Option<ConfigRate>,
    pub config_crimson_witch_of_flames: Option<ConfigLevel>,
    pub config_heart_of_depth: Option<ConfigRate>,
    pub config_husk_of_opulent_dreams: Option<ConfigLevel>,
    pub config_instructor: Option<ConfigRate>,
    pub config_lavawalker: Option<ConfigRate>,
    pub config_martial_artist: Option<ConfigRate>,
    pub config_noblesse_oblige: Option<ConfigRate>,
    pub config_pale_flame: Option<ConfigPaleFlame>,
    pub config_retracing_bolide: Option<ConfigRate>,
    pub config_shimenawas_reminiscence: Option<ConfigRate>,
    pub config_tenacity_of_the_millelith: Option<ConfigRate>,
    pub config_thundersoother: Option<ConfigRate>,
}

impl ArtifactConfigInterface {
    pub fn to_config(self) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: self.config_archaic_petra.unwrap_or_default(),
            config_berserker: self.config_berserker.unwrap_or_default(),
            config_blizzard_strayer: self.config_blizzard_strayer.unwrap_or(Default::default()),
            config_bloodstained_chivalry: self.config_bloodstained_chivalry.unwrap_or(Default::default()),
            config_brave_heart: self.config_brave_heart.unwrap_or(Default::default()),
            config_crimson_witch_of_flames: self.config_crimson_witch_of_flames.unwrap_or(Default::default()),
            config_heart_of_depth: self.config_heart_of_depth.unwrap_or(Default::default()),
            config_husk_of_opulent_dreams: self.config_husk_of_opulent_dreams.unwrap_or(Default::default()),
            config_instructor: self.config_instructor.unwrap_or(Default::default()),
            config_lavawalker: self.config_lavawalker.unwrap_or(Default::default()),
            config_martial_artist: self.config_martial_artist.unwrap_or(Default::default()),
            config_noblesse_oblige: self.config_noblesse_oblige.unwrap_or(Default::default()),
            config_pale_flame: self.config_pale_flame.unwrap_or(Default::default()),
            config_retracing_bolide: self.config_retracing_bolide.unwrap_or(Default::default()),
            config_shimenawas_reminiscence: self.config_shimenawas_reminiscence.unwrap_or(Default::default()),
            config_tenacity_of_the_millelith: self.config_tenacity_of_the_millelith.unwrap_or(Default::default()),
            config_thundersoother: self.config_thundersoother.unwrap_or(Default::default())
        }
    }
}