use person::SkillGoals;

use crate::{gamedata::*, save::*};

#[skyline::from_offset(0x001db680)]
pub fn is_monastery() -> bool;

#[skyline::from_offset(0x0040c420)]
pub fn get_item_data_from_index(index: i16) -> &'static item::ItemData;

#[repr(C)]
pub struct BattleSkill {
    pub unit: Option<&'static BattleUnit>,
    pub check_ability: i32,
    pub skill_property: i32,
    pub battle_side: [u64; 2],
    pub battle_detail: u64,
    pub condition_check: i32,
    pub skill_value: i32,
}

#[repr(C)]
pub struct BattleUnit {
    pub inventory: [UnitItem; 6],
    pub battalion: UnitBattalion,
    pub seed: u32,
    pub character: Character,
    pub unk_0x26: i16,
    pub unk_0x28: i16,
    pub unk_0x2a: i16,
    pub exp: i16,
    pub equipped_weapon: i16,
    pub equipped_accessory: i16,
    pub skill_exp: [u16; 12],
    pub level: u8,
    pub class: u8,
    pub hp: u8,
    pub hp_mod: u8,
    pub stats: [u8; 9],
    pub combat_art_unlock: [u8; 10],
    pub ability_unlock: [u8; 30],
    pub equipped_abilities: [u8; 5],
    pub equipped_combat_art: [u8; 3],
    pub item_count: u8,
    pub skill_rank_level: [u8; 11],
    pub current_class_level: u8,
    pub magic_durability: [u8; 12],
    pub learned_magic: [u8; 12],
    pub unit_flag1: u8,
    pub unit_flag2: u8,
    pub unit_flag3: u8,
    pub personal: u8,
    pub costume: u8,
    pub quest: u8,
    pub costume_flag: u8,
    pub unk_0xb3: u8,
    pub cooking_boost: [u8; 4],
    pub junk: [u8; 56],
    pub deploy_slot: u8,
    pub force: u8,
    pub x: u8,
    pub y: u8,
    pub unk_0xf4: u8,
    pub gambit_uses: u8,
    pub movement: u8,
    pub ai_parameters: [u8; 6],
    pub target_bai: [u8; 3],
    pub adjutant_id: i8,
    pub __: [u8; 6],
    pub defeat_text: u8,
    pub acted: u8,
    pub faction: u8,
}

#[derive(Debug)]
#[repr(u16)]
pub enum Character {
    MByleth,
    FByleth,
    Edelgard,
    Dimitri,
    Claude,
    Hubert,
    Ferdinand,
    Linhardt,
    Caspar,
    Bernadetta,
    Dorothea,
    Petra,
    Dedue,
    Felix,
    Ashe,
    Sylvain,
    Mercedes,
    Annette,
    Ingrid,
    Lorenz,
    Raphael,
    Ignatz,
    Lysithea,
    Marianne,
    Hilda,
    Leonie,
    Seteth,
    Flayn,
    Hanneman,
    Manuela,
    Gilbert,
    Alois,
    Catherine,
    Shamir,
    Cyril,
    Jeralt,
    Rhea,
    Sothis,
    //skipped npcs
    Yuri = 1040,
    Balthus = 1041,
    Constance = 1042,
    Hapi = 1043,
    Aelfric = 1044,
    Jeritza = 1045,
    Anna = 1046,
}

#[repr(C)]
pub struct UnitItem {
    pub item_id: i16,
    pub durability: i16,
}

#[repr(C)]
pub struct UnitBattalion {
    pub char_id: i16,
    pub exp: i16,
    pub endurance: i16,
    pub battalion_id: u8,
    pub gambit: u8,
}

#[repr(C)]
pub struct Unit {
    pub inventory: [UnitItem; 6],
    pub battalion: UnitBattalion,
    pub seed: u32,
    pub character: i16,
    pub battles: i16,
    pub victories: i16,
    pub death_map_id: u8,
    pub death_chapter: u8,
    pub exp: i16,
    pub equipped_weapon: i16,
    pub equipped_accessory: i16,
    pub skill_exp: [u16; 11],
    pub current_class_exp: u16,
    pub level: u8,
    pub class: u8,
    pub hp: u8,
    pub hp_mods: u8,
    pub stats: [u8; 9],
    pub combat_art_unlock: [u8; 10],
    pub ability_unlock: [u8; 30],
    pub equipped_abilities: [u8; 5],
    pub equipped_combat_art: [u8; 3],
    pub item_count: u8,
    pub skill_rank_level: [u8; 11],
    pub current_class_level: u8,
    pub magic_durability: [u8; 12],
    pub learned_magic: [u8; 12],
    pub flag: i32,
    pub costume: u8,
    pub quest_related: u8,
    pub playable_flags: u8,
    pub unk_0xb3: u8,
    pub cooking_boosts: [u8; 4],
    pub unk_0xb8: [u8; 6],
    pub learn_abilities_set2: [u8; 2],
    pub unk_0xc0: [u8; 4],
    pub class_related_stuff: UnitClass,
    pub unk_0xee: [u8; 5],
    pub unk_0xf3: [u8; 7],
    pub adjutant_id: u16,
    pub skill_exp2: [u16; 11],
    pub class_exp: [u16; 97],
    pub seeded: u32, // secret seed
    pub __: i16,
    pub deploy_index: i8,
    pub deploy_slot: i8,
    pub skill_level: [u8; 11],
    pub class_level: [u8; 100],
}

#[repr(C)]
pub struct UnitClass {
    pub movitation: u8,
    pub gift_0x1: u8,
    pub skill_instruct: [u8; 11],
    pub unk_0xd: u8,
    pub unk_0xe: u8,
    pub class_unlock_flags: [u8; 8],
    pub goal: u8,
    pub goals_flag: u8,
    pub month_tea_charm: u8,
    pub tea_charm: u8,
    pub monastery_status: u8,
    pub possible_flags: [u8; 2],
    pub class_unlock_flags2: [u8; 3],
    pub recruitment_text: [i16; 4],
}

impl Unit {
    pub fn add_motivation(&self, value: u8) {
        unsafe { unit_add_motivation(self, value) };
    }
    pub fn learn_ability(&self, ability: i32) {
        unsafe { unit_learn_ability(self, ability) };
    }
    pub fn learn_combat_art(&self, combat_art: i32) {
        unsafe { unit_learn_combat_art(&self, combat_art) };
    }
    pub fn learn_from_ranks(&self, param2: u64) {
        unsafe { unit_learn_from_ranks(self, param2) };
    }
    pub fn cert_into_class(&self, class: i32) {
        unsafe { unit_cert_class(&self.class_related_stuff, class, true) };
    }
    pub fn has_class(&self, class: i32) -> bool {
        unsafe { has_cert(&self.class_related_stuff, class) }
    }
    pub fn add_item(&self, item: i32) {
        unsafe { unit_add_item(self, item) };
    }
    pub fn gain_skill_exp(&self, skill: i32, earn: i32) {
        unsafe { unit_gain_skill_exp(self, skill, earn) };
    }
    pub fn get_ability_parameter(&self, ability_parameter: i32) -> i32 {
        unsafe { get_unit_ability_parameter(self, ability_parameter) }
    }

    pub fn grant_battalion(&self, battalion_id: i32, equip: bool) {
        let save = Save::get_instance();
        unsafe {
            grant_unit_battalion(save, self, battalion_id, equip);
        }
    }

    pub fn reclass(&self, class: i32, param3: bool, param4: u64, param5: u64) {
        unsafe { unit_reclass(self, class, param3, param4, param5) };
    }
    pub fn level_up(&self, to_level: i32) {
        unsafe {
            unit_level(self, to_level);
        }
    }
    pub fn set_class_mastered(&self, class: u8) {
        unsafe {
            unit_set_class_level(self, class, true);
        }
    }
    pub fn toggle_budding_talent(&self, skill: u8, enable: bool) {
        unsafe {
            unit_enable_budding_talent(self, skill, enable);
        }
    }
    pub fn has_budding_talent(&self, skill: u8) -> bool {
        unsafe { unit_has_budding_talent(self, skill) }
    }

    pub fn get_goal_skills(&self) -> (i32, i32) {
        let goal = self.class_related_stuff.goal as usize;
        SkillGoals::get_goal_skills(goal)
    }
}

impl BattleUnit {
    pub fn learn_ability(&self, ability: i32) {
        unsafe { unit_learn_ability2(self, ability) };
    }
    pub fn equip_ability(&self, slot: i32, ability: i32) {
        unsafe { unit_equip_ability(self, slot, ability) };
    }
}

#[skyline::from_offset(0x004149f0)]
pub fn has_cert(class: &UnitClass, class_id: i32) -> bool;

#[skyline::from_offset(0x003d2a10)]
pub fn unit_cert_class(class_stuff: &UnitClass, class_id: i32, equip: bool);

#[skyline::from_offset(0x004124f0)]
pub fn unit_learn_ability(unit: &Unit, ability: i32);

#[skyline::from_offset(0x00412b50)]
pub fn unit_equip_ability(unit: &BattleUnit, slot: i32, ability: i32);

#[skyline::from_offset(0x004124f0)]
pub fn unit_learn_ability2(unit: &BattleUnit, ability: i32);

#[skyline::from_offset(0x00412420)]
pub fn unit_learn_combat_art(unit: &Unit, combat_art: i32);

#[skyline::from_offset(0x003d2ac0)]
pub fn unit_reclass(unit: &Unit, class: i32, param3: bool, param4: u64, param5: u64);

#[skyline::from_offset(0x003dc1e0)]
pub fn grant_unit_battalion(save: &Save, unit: &Unit, battalion_id: i32, equip: bool);

#[skyline::from_offset(0x003ddb50)]
pub fn unit_add_item(unit: &Unit, item_id: i32);

#[skyline::from_offset(0x00411710)]
pub fn unit_learn_from_ranks(unit: &Unit, parm2: u64);

#[skyline::from_offset(0x00411590)]
pub fn unit_gain_skill_exp(unit: &Unit, skill_type: i32, earn: i32);

#[skyline::from_offset(0xa7e80)]
pub fn get_unit_ability_parameter(unit: &Unit, ability_index: i32) -> i32;

#[skyline::from_offset(0x3d3020)]
pub fn unit_level(unit: &Unit, level: i32);

#[skyline::from_offset(0x00415420)]
pub fn unit_set_class_level(unit: &Unit, class: u8, level: bool);

#[skyline::from_offset(0x003e0250)]
pub fn unit_add_motivation(unit: &Unit, value: u8);

#[skyline::from_offset(0x00414750)]
fn unit_enable_budding_talent(unit: &Unit, skill: u8, enable: bool);

#[skyline::from_offset(0x0040e460)]
fn unit_has_budding_talent(unit: &Unit, skill: u8) -> bool;

#[skyline::from_offset(0x003d48f0)]
fn character_class_get_skill_from_goals(goal: u8, skill: &mut [i16; 4], param3: i32);
