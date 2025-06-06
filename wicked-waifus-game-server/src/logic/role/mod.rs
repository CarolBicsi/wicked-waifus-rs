use std::collections::HashMap;

use wicked_waifus_protocol::{ArrayIntInt, RoleInfo};

use crate::config;
use crate::logic::utils::growth_utils::get_role_props_by_level;
use crate::logic::utils::load_role_info::load_key_value;
pub use formation::RoleFormation;
use wicked_waifus_commons::time_util;
use wicked_waifus_data::{role_info_data, BasePropertyData};
use wicked_waifus_protocol_internal::{RoleData, RoleStats};
use crate::logic::player::Element;

const MAIN_CHARACTER_MALE_SPECTRO_ID: i32 = 1501;
const MAIN_CHARACTER_FEMALE_SPECTRO_ID: i32 = 1502;
const MAIN_CHARACTER_MALE_HAVOC_ID: i32 = 1605;
const MAIN_CHARACTER_FEMALE_HAVOC_ID: i32 = 1604;
const MAIN_CHARACTER_MALE_AERO_ID: i32 = 1406;
const MAIN_CHARACTER_FEMALE_AERO_ID: i32 = 1408;

const MAIN_CHARACTER_ARRAY: &[i32] = &[
    MAIN_CHARACTER_MALE_SPECTRO_ID,
    MAIN_CHARACTER_FEMALE_SPECTRO_ID,
    MAIN_CHARACTER_MALE_HAVOC_ID,
    MAIN_CHARACTER_FEMALE_HAVOC_ID,
    MAIN_CHARACTER_MALE_AERO_ID,
    MAIN_CHARACTER_FEMALE_AERO_ID,
];

mod formation;
pub struct Role {
    pub role_id: i32,
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub breakthrough: i32,
    pub skill_map: HashMap<i32, i32>,
    pub star: i32,
    pub favor: i32,
    pub create_time: u32,
    pub equip_weapon: i32,
    pub skin_id: i32,
    pub resonant_chain_group_index: i32,
    pub hp: i32,
    pub energy: i32,
    pub special_energy_1: i32,
    pub special_energy_2: i32,
    pub special_energy_3: i32,
    pub special_energy_4: i32,
    pub element_energy: i32,
    pub favor_level: i32,
    pub favor_exp: i32,
}

impl Role {
    #[inline(always)]
    pub fn get_all_roles_except_mc() -> Vec<i32> {
        role_info_data::iter()
            .filter(|role| (role.role_type == 1 && !MAIN_CHARACTER_ARRAY.contains(&role.id)) || role.id == 5101)
            .map(|role| role.id)
            .collect()
    }

    #[inline(always)]
    pub fn get_mc_id_based_on_sex(sex: i32, current_element: Element) -> i32 {
        match current_element {
            Element::Glacio => unreachable!(),
            Element::Fusion => unreachable!(),
            Element::Electro => unreachable!(),
            Element::Aero => match sex {
                0 => MAIN_CHARACTER_MALE_AERO_ID,
                1 => MAIN_CHARACTER_FEMALE_AERO_ID,
                _ => unreachable!(),
            }
            Element::Spectro => match sex {
                0 => MAIN_CHARACTER_FEMALE_SPECTRO_ID,
                1 => MAIN_CHARACTER_MALE_SPECTRO_ID,
                _ => unreachable!(),
            }
            Element::Havoc => match sex {
                0 => MAIN_CHARACTER_MALE_HAVOC_ID,
                1 => MAIN_CHARACTER_FEMALE_HAVOC_ID,
                _ => unreachable!(),
            }
        }
    }

    #[inline(always)]
    pub fn get_mc_based_on_sex(sex: i32, current_element: Element) -> Self {
        Role::new(Self::get_mc_id_based_on_sex(sex, current_element))
    }

    pub fn get_mc_unlock_variations(unlocked_elements: Vec<Element>) -> Vec<i32> {
        let mut variations: Vec<i32> = Vec::new();
        for element in unlocked_elements {
            match element {
                Element::Glacio => {},
                Element::Fusion => {},
                Element::Electro => {},
                Element::Aero => {
                    variations.push(MAIN_CHARACTER_MALE_AERO_ID);
                    variations.push(MAIN_CHARACTER_FEMALE_AERO_ID);
                },
                Element::Spectro => {
                    variations.push(MAIN_CHARACTER_MALE_SPECTRO_ID);
                    variations.push(MAIN_CHARACTER_FEMALE_SPECTRO_ID);
                },
                Element::Havoc => {
                    variations.push(MAIN_CHARACTER_MALE_HAVOC_ID);
                    variations.push(MAIN_CHARACTER_FEMALE_HAVOC_ID);
                },
            }
        }
        variations
    }

    pub fn new(role_id: i32) -> Self {
        let data = role_info_data::iter().find(|d| d.id == role_id).unwrap();

        let default_unlocks = &config::get_config().default_unlocks;
        let (level, breakthrough) = if default_unlocks.unlock_all_roles_max_level {
            (data.max_level, 6)
        } else {
            (1, 0)
        };
        let resonant_chain_group_index = if default_unlocks.unlock_all_roles_all_sequences {
            6
        } else {
            0
        };
        let base_stats = &get_role_props_by_level(role_id, level, breakthrough);

        // weapon
        let equip_weapon = match role_id { // this is not show on character equiped menu
            1402 => 21020046, // Yangyang 
            1202 => 21030015, // Chixia 
            1503 => 21050015, // Verina 
            1501 => 21020046, // Rover: Spectro 
            1102 => 21020046, // Sanhua 
            1601 => 21010036, // Taoqi 
            1502 => 21020046, // Rover: Spectro 
            1103 => 21050015, // Baizhi 
            1203 => 21050015, // Encore 
            1602 => 21020046, // Danjin 
            1403 => 21030015, // Aalto 
            1404 => 21010016, // Jiyan 
            1204 => 21030015, // Mortefi 
            1603 => 21020026, // Camellya 
            1301 => 21010036, // Calcharo 
            1302 => 21050016, // Yinlin 
            1104 => 21040015, // Lingyang 
            1303 => 21040015, // Yuanwu 
            1604 => 21020046, // Rover: Havoc 
            1605 => 21020046, // Rover: Havoc 
            1504 => 21010036, // Lumi 
            1505 => 21050036, // Shorekeeper 
            1405 => 21040015, // Jianxin 
            1304 => 80080007, // Jinhsi 
            1105 => 21050026, // Zhezhi 
            1205 => 21020016, // Changli 
            1305 => 21040016, // Xiangli Yao 
            1106 => 21040015, // Youhu 
            1606 => 21040026, // Roccia 
            1107 => 21030016, // Carlotta 
            1408 => 21020046, // Rover: Aero 
            1406 => 21020046, // Rover: Aero 
            1206 => 21020036, // Brant 
            1506 => 21050046, // Phoebe 
            1607 => 21050056, // Cantarella 
            1507 => 21040036, // Zani 
            1407 => 21030026, // Ciaccona 
            1207 => 21010036, // Lupa 
            1409 => 21020056, // Cartethyia 
          _ => data.init_weapon_item_id,
        };

        // skin
        let skin_id = match role_id {
          1205 => 81011205, // changli skin
          1107 => 81011107, // carotta skin
          1304 => 81011304, // Jinhsi skin
          1102 => 81011102, // Sanhua skin
          _ => data.skin_id,
        };


        Self {
            role_id,
            name: String::with_capacity(0),
            level,
            exp: 0, // TODO: Compute based on level??
            breakthrough,
            skill_map: HashMap::new(), // TODO!
            star: 0,
            favor: 0,
            create_time: time_util::unix_timestamp() as u32,
            equip_weapon: equip_weapon,
            skin_id: skin_id,
            resonant_chain_group_index,
            hp: base_stats.life,
            energy: base_stats.energy,
            special_energy_1: base_stats.special_energy_1,
            special_energy_2: base_stats.special_energy_2,
            special_energy_3: base_stats.special_energy_3,
            special_energy_4: base_stats.special_energy_4,
            element_energy: base_stats.element_energy,
            favor_level: 0,
            favor_exp: 0,
        }
    }

    pub fn get_base_properties(&self) -> BasePropertyData {
        // Overwrite dynamic attributes with stores values
        let mut base_stats = get_role_props_by_level(self.role_id, self.level, self.breakthrough);
        // TODO: Integrity check, value has to be between 0 and max
        base_stats.life = base_stats.life_max;
        base_stats.energy = base_stats.energy_max;
        base_stats.special_energy_1 = base_stats.special_energy_1_max;
        base_stats.special_energy_2 = base_stats.special_energy_2_max;
        base_stats.special_energy_3 = base_stats.special_energy_3_max;
        base_stats.special_energy_4 = base_stats.special_energy_4_max;
        base_stats.element_energy = base_stats.element_energy_max;
        base_stats
    
    }

    pub fn to_protobuf(&self) -> RoleInfo {
        let base_prop: HashMap<i32, i32> = load_key_value(&self.get_base_properties());
        RoleInfo {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            create_time: self.create_time,
            skills: self
                .skill_map
                .iter()
                .map(|(k, v)| ArrayIntInt { key: *k, value: *v })
                .collect(),
            base_prop: base_prop
                .iter()
                .map(|(&k, &v)| ArrayIntInt { key: k, value: v })
                .collect(),
            skin_id: self.skin_id,
            resonant_chain_group_index: self.resonant_chain_group_index,
            ..Default::default()
        }
    }

    pub fn load_from_save(data: RoleData) -> (i32, Self) {
        (
            data.role_id,
            Self {
                role_id: data.role_id,
                name: data.name,
                level: data.level,
                exp: data.exp,
                breakthrough: data.breakthrough,
                skill_map: data.skill_map,
                star: data.star,
                favor: data.favor,
                create_time: data.create_time,
                equip_weapon: data.equip_weapon,
                skin_id: data.skin_id,
                resonant_chain_group_index: data.resonant_chain_group_index,
                hp: data.stats.unwrap().hp,
                energy: data.stats.unwrap().energy,
                special_energy_1: data.stats.unwrap().special_energy_1,
                special_energy_2: data.stats.unwrap().special_energy_2,
                special_energy_3: data.stats.unwrap().special_energy_3,
                special_energy_4: data.stats.unwrap().special_energy_4,
                element_energy: data.stats.unwrap().element_energy,
                favor_level: data.favor_level,
                favor_exp: data.favor_exp,
            },
        )
    }

    pub fn build_save_data(&self) -> RoleData {
        RoleData {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            skill_map: self.skill_map.clone(),
            // phantom_map: Default::default(),
            star: self.star,
            favor: self.favor,
            create_time: self.create_time,
            // cur_model: 0,
            // models: vec![],
            // skill_node_state: vec![],
            resonant_chain_group_index: self.resonant_chain_group_index,
            equip_weapon: self.equip_weapon,
            skin_id: self.skin_id,
            stats: Some(RoleStats {
                hp: self.hp,
                energy: self.energy,
                special_energy_1: self.special_energy_1,
                special_energy_2: self.special_energy_2,
                special_energy_3: self.special_energy_3,
                special_energy_4: self.special_energy_4,
                element_energy: self.element_energy,
            }),
            favor_level: self.favor_level,
            favor_exp: self.favor_exp,
            ..Default::default()
        }
    }
}
