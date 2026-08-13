use crate::{gamedata::*, unit::*};

#[repr(C)]
pub struct Save {
    pub items: [UnitItem; 400],
    pub item_count: u32,
    pub unit_list: [Unit; 60],
    junk: [u8; 106988],
    pub player: PlayerData,
    pub activities: ActivityData,
}

#[repr(C)]
pub struct PlayerData {
    junk: [u8; 4208],
    pub playtime: u32,
    pub money: u32,
    pub unk: u32,
    pub chapter: i32,
    pub support_exp: [u16; 270],
    pub difficulty: u8,
    pub game_mode: u8,
    pub route: u8,
    pub timeskip: bool,
    pub map_id: u8,
    pub monastery: i8,
    pub flags: [u8; 271],
    junk2: [u8; 2839],
}

#[repr(C)]
pub struct ActivityData {
    pub unk_0x0: [u8; 8],
    pub battle_rng_uns: i32,
    pub renown: u32,
    pub unk_0x10: [u8; 2],
    pub prof_exp: u16,
    pub support_queue: [Character; 7],
    pub group_task_characters: [Character; 2],
    pub current_day: u8,
    pub explore_points: u8,
    pub lesson_points: u8,
    pub battle_points: u8,
    pub unk_0x2a: u8, //reichard called this 'lol'
    pub saint_statue_upgrades: [u8; 4],
    pub battalion_shop_lvl: u8,
    pub battalion_shop_stock: [u8; 200],
    pub merchant_shop_stock: [u8; 100],
    pub anna_secret_shop_stock: [u8; 50],
    pub weapon_shop_lvl: u8,
    pub weapon_shop_stock: [u8; 200],
    pub item_shop_lvl: u8,
    pub equip_shop_stock: [u8; 50],
    pub item_shop_stock: [u8; 200],
    pub unk_0x352: [u8; 10],
    pub quest_todo_uns: [u8; 10],
    pub monastery_region_array: [u8; 100],
    pub seeds_in_greenhouse: [u8; 5], //MiscItem enum
    pub cultivation_lvl: u8,          //975
    pub prof_exp_seed: u8,            //MiscItem
    pub garden_event: u8,
    pub advice_box_use_count: u8,
    pub advice_box_avail_count: u8,
    pub advice_box_entry_completed: [u8; 13],
    pub unk_0x3e1: [u8; 19],
    pub quest_status: [u8; 151], //1012
    pub avail_battles: [u8; 3],
    pub aux_battle_reqards: [u8; 3],
    pub aux_unit_indexes: [u16; 3],
    pub questid: u8,
    pub preskip_questions_asked: [u8; 13],
    pub postskip_questions_asked: [u8; 13],
    pub faculty_training_flags: [u8; 7],
    pub unk_0x4b9: [u8; 26], //1209
    pub spot_data_flags1: [u8; 38],
    pub spot_data_flags2: [u8; 38],
    pub spot_data_flags3: [u8; 38],
    pub active_paralogue_flags: [u8; 3],
    pub misc_items_removed: [u8; 2], //MiscItem
    pub misc_items_removed_count: [u8; 2],
    pub monastery_location: [u8; 13],
    pub group_task: GroupTask, //1369
    pub group_task_dialogue_index: u8,
    pub monastery_activities_completed_flags1: u8,
    pub monastery_activities_completed_flags2: u8,
    pub playlog_work: u8,
    pub playlog_lecture: u8,
    pub playlog_battle: u8,
    pub playlog_rest: u8,
    pub playlog_train_mount: u8,
    pub playlog_choir_practice: u8,
    pub playlog_share_a_meal: u8,
    pub playlog_cooking_together: u8,
    pub playlog_drill: u8,
    pub playlog_tea_party: u8,
    pub playlog_scout: u8,
    pub playlog_unk1: u8,
    pub playlog_unk2: u8,
    pub unk_0x56a: u8,
    pub unk_0x56b: u8,
    pub influencer_upg_lvl: u8,
    pub pagan_altar_weapon_shop_stock: [u8; 50],
    pub pagan_altar_equip_shop_stock: [u8; 30],
    pub pagan_altar_item_shop_stock: [u8; 20],
    pub misc_item_shop_stock: [u8; 30],
}

#[repr(u8)]
pub enum ProfRank {
    E,
    Eplus,
    D,
    Dplus,
    C,
    Cplus,
    B,
    Bplus,
    A,
    Aplus,
}

#[repr(u8)]
pub enum GroupTask {
    Stable = 0,
    Weeding = 3,
    SkyWatch = 6,
}

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

impl Save {
    pub fn get_instance() -> &'static mut Save {
        let save = crate::gamedata::offset_to_addr::<&mut Save>(0x01b12190);
        unsafe {
            return *save;
        }
    }
    pub fn get_activity_data() -> &'static ActivityData {
        let save = Self::get_instance();
        return &save.activities;
    }

    pub fn get_player_data() -> &'static PlayerData {
        let save = Self::get_instance();
        return &save.player;
    }

    pub fn setup_unit_recruitment(&self, character_id: i32, map_id: i32, battalion: bool) {
        let save = Self::get_instance();
        unsafe {
            setup_for_recruitment(save, character_id, map_id, battalion);
        }
    }

    pub fn recruit_character(&self, character_id: i32) {
        let save = Self::get_instance();
        unsafe {
            save_data_recruitment(save, character_id);
        }
    }

    pub fn get_character(character_id: i32) -> Option<&'static mut Unit> {
        let save = Self::get_instance();
        unsafe { get_unit_from_save(save, character_id as u32) }
    }

    pub fn scenario_recruitment(&self, character_id: i32, map_id: i32, mission_assistance: bool) {
        let save = Self::get_instance();
        unsafe { scenario_recruitment(save, character_id, map_id, mission_assistance) };
    }

    pub fn get_current_flow() -> &'static crate::gamedata::FlowEntry {
        let save = Self::get_instance();
        let route = save.player.route as i32;
        let chapter = save.player.chapter;
        unsafe { get_fixed_flow_entry(route, chapter) }
    }

    pub fn set_route(&self, route: i32) {
        unsafe { save_set_route(self, route) };
    }
}

impl PlayerData {
    pub fn set_flag(&self, flag: i32, status: bool) {
        unsafe { set_flag(self, flag, status) };
    }
    pub fn get_flag(&self, flag: i32) -> bool {
        unsafe { get_flag(self, flag) }
    }
}

impl ActivityData {
    pub fn get_instance() -> &'static Self {
        let save = Save::get_instance();
        &save.activities
    }
    pub fn get_quest_status(&self, quest: i32) -> i32 {
        unsafe { activity_get_quest_status(self, quest) }
    }

    pub fn set_quest_status(&self, quest: i32, status: i32) {
        unsafe { set_quest_status(self, quest, status) };
    }
    pub fn set_paralogue_status(&self, map_id: i32, status: bool) {
        unsafe { set_paralogue(self, map_id, status) };
    }
    pub fn is_paralogue_active(&self, paralogue: i32) -> bool {
        unsafe { activity_is_paralogue_active(self, paralogue) }
    }
    pub fn get_instruction_bonus(&self, skill: i32) -> i32 {
        unsafe { activity_instruction_bonus(self, skill) }
    }
    pub fn get_prof_rank(&self) -> ProfRank {
        unsafe {
            return calc_professor_rank(self);
        }
    }
}

#[skyline::from_offset(0x003da660)]
fn save_set_route(save: &Save, route: i32);

#[skyline::from_offset(0x003d0dc0)]
fn setup_for_recruitment(savedata: &Save, char_id: i32, map_id: i32, param: bool);

#[skyline::from_offset(0x0003d0b80)]
fn save_data_recruitment(savedata: &Save, char_ide: i32);

#[skyline::from_offset(0x03caf30)]
fn get_unit_from_save(save: &Save, char_id: u32) -> Option<&'static mut Unit>;

#[skyline::from_offset(0x003cafc0)]
fn scenario_recruitment(save: &Save, char_id: i32, map_id: i32, mission_assistance: bool);

#[skyline::from_offset(0x003d8620)]
fn get_fixed_flow_entry(route: i32, chapter: i32) -> &'static FlowEntry;

// PlayerData
#[skyline::from_offset(0x003d8740)]
fn set_flag(player: &PlayerData, flag: i32, status: bool);

#[skyline::from_offset(0x003de440)]
fn get_flag(player: &PlayerData, flag: i32) -> bool;
// ActivityData
#[skyline::from_offset(0x003ddef0)]
fn set_quest_status(activities: &ActivityData, quest: i32, status: i32);

#[skyline::from_offset(0x003f17a0)]
fn set_paralogue(activities: &ActivityData, map_id: i32, set: bool);

#[skyline::from_offset(0x003eeb30)]
fn activity_get_quest_status(activities: &ActivityData, quest: i32) -> i32;

#[skyline::from_offset(0x003f10c0)]
pub fn activity_is_paralogue_active(activities: &ActivityData, paralogue: i32) -> bool;

#[skyline::from_offset(0x003f2f20)]
fn activity_instruction_bonus(activities: &ActivityData, skill: i32) -> i32;

#[skyline::from_offset(0x003e1cb0)]
fn calc_professor_rank(activity: &ActivityData) -> ProfRank;
