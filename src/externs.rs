extern "C"{
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;

    #[link_name = "\u{1}_ZN3app14sv_information11is_ready_goEv"]
    pub fn is_ready_go() -> bool;

    #[link_name = "\u{1}_ZN3app12item_manager22get_num_of_active_itemENS_8ItemKindE"]
    pub fn get_num_of_active_item(item_kind: i32) -> u64;

    #[link_name = "\u{1}_ZN3app14sv_information27get_remaining_time_as_frameEv"]
    pub fn get_remaining_time_as_frame() -> u32;
}