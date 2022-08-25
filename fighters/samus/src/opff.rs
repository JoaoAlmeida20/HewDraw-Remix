// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn missile_land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

// Shinespark charge
unsafe fn shinespark_charge(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if *FIGHTER_STATUS_KIND_RUN == status_kind && frame > 30.0 {
        if  !VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
            VarModule::on_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
        }
    }

    if VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
        // Glow blue during speed boost
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.172, /* Green */ y: 0.439, /* Blue */ z: 0.866, /* Alpha */ w: 0.8};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
        
        // Allow for walljumps with jump button to avoid problems with losing speed boost by flicking back
        let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        if ((touch_right || touch_left) && boma.is_input_jump()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }
}

// Shinespark Reset
unsafe fn shinespark_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if !boma.is_motion(Hash40::new("attack_dash")) {
        VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_USED);
    }
    if !(([*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_RUN_BRAKE,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_JUMP_SQUAT,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)
            && boma.is_stick_forward()) 
        || status_kind == *FIGHTER_STATUS_KIND_WALL_JUMP) {
            VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
        
            // Dont disable color if the shinespark was stored as Samus should still be glowing
            if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) == 0.0 {
                ColorBlendModule::cancel_main_color(boma, 0);
            }
    }
    // Reset storage on death
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 0.0);
        ColorBlendModule::cancel_main_color(boma, 0);
    }
}

// Shinespark storage
unsafe fn shinespark_storage(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    // Decrement shinespark timer and glow purple when its stored
    if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
        VarModule::sub_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 1.0);
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.2};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.75, /* Green */ y: 0.25, /* Blue */ z: 0.925, /* Alpha */ w: 0.8};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    }

    // Begin timer of 5 seconds for storing shinespark with crouch (must crouch for 5+ frames)
    if *FIGHTER_STATUS_KIND_SQUAT == status_kind && boma.motion_frame() >= 5.0 && VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY)
        && VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) == 0.0 {
        VarModule::set_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 300.0);
        VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
    }
}

// Shinespark air
unsafe fn shinespark_air(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0
    && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) 
        || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
    && boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR)
    && boma.motion_frame() <= 6.0 {
        MotionModule::change_motion(boma, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    }
}

// Morph Ball Crawl
// PUBLIC
pub unsafe fn morphball_crawl(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    /* if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind) {
        if frame >= 31.0 {
            if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW))
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 12.0, 1.0, 1.0);
            }
        }
    } */

    if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
    *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A].contains(&status_kind) {
        // Place bomb by pressing Attack
        if boma.is_button_trigger(Buttons::Attack | Buttons::AttackRaw)
        && frame <= 43.0
        && VarModule::get_int(boma.object(), vars::samus::instance::BOMB_COUNTER) < 8 {
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
            ArticleModule::shoot_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            VarModule::inc_int(boma.object(), vars::samus::instance::BOMB_COUNTER);
        }
        // Exit morphball by pressing Special
        if boma.is_button_trigger(Buttons::SpecialAll)
        && 30.0 <= frame
        && frame <= 42.0 {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 44.0, 1.0, 1.0);
        }
        // Stay in morphball after a bomb jump
        if frame == 12.0
        && [*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A].contains(&status_kind) {
                if boma.is_situation(*SITUATION_KIND_GROUND) {
                    StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, false);
                }
                else if boma.is_situation(*SITUATION_KIND_AIR) {
                    StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, false);
                }
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 12.0, 1.0, 1.0);
        }
        // Loop before end of morphball
        else if 42.0 < frame
        && frame <= 43.0 {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 30.0, 1.0, 1.0);
        }
        // Allow jumping and double jumping in morphball
        if boma.is_input_jump() {
            let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                let mini_jump_y = WorkModule::get_param_float(boma, hash40("mini_jump_y"), 0);
                let jumpSpeed = Vector3f{x: 0.0, y: (air_accel_y * (mini_jump_y / (0.5 * air_accel_y)).sqrt()), z: 0.0};
                KineticModule::add_speed(boma, jumpSpeed);
                WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
            else if boma.is_situation(*SITUATION_KIND_AIR)
            && boma.get_num_used_jumps() < boma.get_jump_count_max() {
                let jump_aerial_y = WorkModule::get_param_float(boma, hash40("jump_aerial_y"), 0);
                let jumpSpeed = Vector3f{x: 0.0, y: (air_accel_y * (jump_aerial_y / (0.5 * air_accel_y)).sqrt()), z: 0.0};
                KineticModule::add_speed(boma, jumpSpeed);
                WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }

    if VarModule::get_int(boma.object(), vars::samus::instance::BOMB_COUNTER) != 0
    && (!boma.is_situation(*SITUATION_KIND_AIR) ||
        boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY
        ])) {
        VarModule::set_int(boma.object(), vars::samus::instance::BOMB_COUNTER, 0);
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
        missile_land_cancel(fighter, &mut *info.boma, info.id, info.status_kind, info.situation_kind);
        morphball_crawl(&mut *info.boma, info.status_kind, info.frame);
        nspecial_cancels(&mut *info.boma, info.status_kind, info.situation_kind);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    shinespark_charge(boma, id, status_kind, frame);
    shinespark_reset(boma, id, status_kind);
    shinespark_storage(boma, id, status_kind);
    shinespark_air(boma);
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