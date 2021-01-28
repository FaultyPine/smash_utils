use smash::lib::lua_const::*;
use smash::app::{self, lua_bind::*};

#[inline(always)]
pub unsafe fn get_player_number(boma: &mut app::BattleObjectModuleAccessor) -> usize {
    WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

#[inline(always)]
pub unsafe fn get_jump_count(boma: &mut app::BattleObjectModuleAccessor) -> i32 {
    WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT)
}

#[inline(always)]
pub unsafe fn get_jump_count_max(boma: &mut app::BattleObjectModuleAccessor) -> i32 {
    WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
}

#[inline(always)]
pub unsafe fn used_airdodge(boma: &mut app::BattleObjectModuleAccessor) -> bool {
    WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
}

#[inline(always)]
pub unsafe fn get_costume_slot(boma: &mut app::BattleObjectModuleAccessor) -> i32 {
    WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR)
}

pub fn get_fighter_manager() -> *mut app::FighterManager {
    unsafe {
        *(crate::FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager)
    }
}

pub fn get_fighter_information(module_accessor: &mut app::BattleObjectModuleAccessor) -> Option<*mut app::FighterInformation> {
    unsafe {
        if !smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            return None;
        }
        let entry_id = app::FighterEntryID(WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID));
        let fighter_information = FighterManager::get_fighter_information(get_fighter_manager(), entry_id) as *mut app::FighterInformation;
        Some(fighter_information)
    }
}

pub fn get_module_accessor(fighter_id: i32) -> *mut app::BattleObjectModuleAccessor {
    let entry_id = app::FighterEntryID(fighter_id);
    unsafe {
        let fighter_entry = FighterManager::get_fighter_entry(get_fighter_manager(), entry_id) as *mut app::FighterEntry;
        let current_fighter_id = FighterEntry::current_fighter_id(fighter_entry);
        app::sv_battle_object::module_accessor(current_fighter_id as u32)
    }
}