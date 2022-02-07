// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn land_cancel_and_b_reverse(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR{
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if frame < 5.0 {
                if stick_x * facing < 0.0 {
                    PostureModule::reverse_lr(boma);
                    PostureModule::update_rot_y_lr(boma);
                    if frame > 1.0 && frame < 5.0 &&  !VarModule::is_flag(boma.object(), vars::common::B_REVERSED) {
                        let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                        KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        VarModule::on_flag(boma.object(), vars::common::B_REVERSED);
                    }
                }
            }
        }
    }
}

// Shinkespark charge
unsafe fn shinespark_charge(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if *FIGHTER_STATUS_KIND_RUN == status_kind && frame > 30.0 {
        if  !VarModule::is_flag(boma.object(), vars::samus::SHINESPARK_READY) {
            VarModule::on_flag(boma.object(), vars::samus::SHINESPARK_READY);
        }
    }

    // Glow blue during "speed boost"
    if VarModule::is_flag(boma.object(), vars::samus::SHINESPARK_READY) {
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.172, /* Green */ y: 0.439, /* Blue */ z: 0.866, /* Alpha */ w: 0.8};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    }
}

// Shinkespark Reset
unsafe fn shinespark_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if ![*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_TURN_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_SQUAT].contains(&status_kind) {
            VarModule::off_flag(boma.object(), vars::samus::SHINESPARK_READY);
            VarModule::off_flag(boma.object(), vars::samus::SHINESPARK_USED);
        
            // Dont disable color if the shinespark was stored as Samus should still be glowing
            if VarModule::get_float(boma.object(), vars::samus::SHINESPARK_TIMER) == 0.0 {
                ColorBlendModule::cancel_main_color(boma, 0);
            }
    }
    // Reset storage on death
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_float(boma.object(), vars::samus::SHINESPARK_TIMER, 0.0);
        ColorBlendModule::cancel_main_color(boma, 0);
    }
}

// Shinespark storage
unsafe fn shinespark_storage(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    // Decrement shinespark timer and glow purple when its stored
    if VarModule::get_float(boma.object(), vars::samus::SHINESPARK_TIMER) > 0.0 {
        VarModule::sub_float(boma.object(), vars::samus::SHINESPARK_TIMER, 1.0);
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.75, /* Green */ y: 0.25, /* Blue */ z: 0.925, /* Alpha */ w: 0.8};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    }

    // Begin timer of 5 seconds for storing shinespark with crouch
    if *FIGHTER_STATUS_KIND_SQUAT == status_kind && VarModule::is_flag(boma.object(), vars::samus::SHINESPARK_READY)
        && VarModule::get_float(boma.object(), vars::samus::SHINESPARK_TIMER) == 0.0 {
        VarModule::set_float(boma.object(), vars::samus::SHINESPARK_TIMER, 300.0);
        VarModule::off_flag(boma.object(), vars::samus::SHINESPARK_READY);
        VarModule::off_flag(boma.object(), vars::samus::SHINESPARK_USED);
    }
}

// Morph Ball Crawl
// PUBLIC
pub unsafe fn morphball_crawl(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind) {
        if frame >= 31.0 {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 12.0, 1.0, 1.0);
            }
        }
    }
}

pub unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "Rust" fn common_samus(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        land_cancel_and_b_reverse(&mut *info.boma, info.id, info.status_kind, info.situation_kind, info.stick_x, info.facing, info.frame);
        morphball_crawl(&mut *info.boma, info.status_kind, info.frame);
        nspecial_cancels(&mut *info.boma, info.status_kind, info.situation_kind);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    shinespark_charge(boma, id, status_kind, frame);
    shinespark_reset(boma, id, status_kind);
    shinespark_storage(boma, id, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_SAMUS )]
pub fn samus_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		samus_frame(fighter);
        common_samus(fighter);
    }
}

pub unsafe fn samus_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}