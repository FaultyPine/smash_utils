#![feature(proc_macro_hygiene)]
#![feature(asm)]

#![allow(dead_code)]
#![allow(unused_imports)]

mod externs;
mod debug;
mod gameplay;

use smash::{lib, app, phx, hash40};

pub const DEFAULT_VEC3: phx::Vector3f = phx::Vector3f {x: 0.0, y: 0.0, z: 0.0};
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;

#[inline(always)]
pub unsafe fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { min } else if x < max { x } else { max }
}

#[inline(always)]
pub fn in_range(num: f32, lower: f32, upper: f32) -> bool{
    num>lower && num<upper
}

#[inline(always)]
pub unsafe fn clear_buffered_action(flag: i32, cmd: i32) -> i32{
    flag & !(flag & cmd)
}

#[inline(always)]
pub unsafe fn add_buffered_action(flag: i32, cmd: i32) -> i32{
    flag | cmd
}

#[inline(always)]
pub unsafe fn compare_cat(cat: i32, fighter_pad_cmd_flag: i32) -> bool{
    (cat & fighter_pad_cmd_flag) != 0
}

pub fn get_remaining_time_as_seconds() -> u32 {
    unsafe { externs::get_remaining_time_as_frame() / 60 }
}

pub fn get_random_int(max: i32) -> i32 {
    unsafe { smash::app::sv_math::rand(smash::hash40("fighter"), max) }
}

//Status script replacer (thanks Peter)
pub unsafe fn sv_replace_status_func(l2c_agentbase: &lib::L2CAgent, status_kind: i32, key: i32, func: u64) {
    let l2c_agentbase = l2c_agentbase as *const lib::L2CAgent as u64 ;
    let status_kind = status_kind as u64;
    let key = key as u64;

    let unk48 = *((l2c_agentbase + 0x48) as *mut u64);

    let x8: u64 = 0xb0 * status_kind + unk48;
    let x9: u64 = key << 32 >> 29;
    *((x8 + x9) as *mut u64) = func;
}


#[skyline::main(name = "smash_utils")]
pub unsafe fn main() {
    skyline::nn::ro::LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
    );
}