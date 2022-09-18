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

unsafe fn change_missiles(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
    && boma.motion_frame() <= 6.0
    && boma.is_button_on(Buttons::AttackAll) {
        if boma.is_situation(*SITUATION_KIND_GROUND) {
            boma.change_status_req(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, false);
        }
        else {
            boma.change_status_req(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, false);
        }
    }
}

unsafe fn aim_gun(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {

    let regular_missile_condition = boma.is_status_one_of(&[
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A])
        && 10.0 <= boma.motion_frame()
        && boma.motion_frame() <= 32.0;

    let super_missile_condition = boma.is_status_one_of(&[
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A])
        && 10.0 <= boma.motion_frame()
        && boma.motion_frame() <= 27.0;

    let charge_shot_condition = 
        boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H)
        || (boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F)
            && boma.motion_frame() <= 22.0)
        || (boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N)
            && boma.motion_frame() <= 12.0);

    // Rotation is flipped around for grounded super missiles for some reason
    let flip_y = if boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G) {
        -1.0
    }
    else {
        1.0
    };

    if super_missile_condition
    || charge_shot_condition {
        let prev_angle = VarModule::get_float(boma.object(), vars::samus::instance::AIM_ANGLE);
        let angle = if boma.stick_x() != 0.0 {
            (boma.stick_y() / boma.stick_x().abs()).atan().to_degrees()
        }
        else {
            boma.stick_y() * 90.0
        }.clamp(prev_angle - 15.0, prev_angle + 15.0);
        VarModule::set_float(boma.object(), vars::samus::instance::AIM_ANGLE, angle);

        if super_missile_condition {
            fighter.set_joint_rotate("armr", Vector3f::new(0.0, 0.0, angle.clamp(-45.0, 45.0) * flip_y));
    
            if angle.abs() > 45.0
            && boma.is_situation(*SITUATION_KIND_AIR) {
                fighter.set_joint_rotate("shoulderr", Vector3f::new(0.0, 0.0, (angle - (45.0 * angle.signum()))));
            }
        }
        else if charge_shot_condition {
            fighter.set_joint_rotate("armr", Vector3f::new(0.0, 0.0, angle.clamp(-45.0, 45.0)));
            if boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H) {
                fighter.set_joint_rotate("arml", Vector3f::new(0.0, 0.0, angle.clamp(-45.0, 45.0)));
            }
    
            if angle.abs() > 45.0
            && boma.is_situation(*SITUATION_KIND_AIR) {
                fighter.set_joint_rotate("waist", Vector3f::new(0.0, 0.0, (angle - (45.0 * angle.signum())) * -1.0));
            }
        }
    }
    // Interpolate back to default rotations
    else {
        let prev_angle = VarModule::get_float(boma.object(), vars::samus::instance::AIM_ANGLE);
        let angle = 0.0_f32.clamp(prev_angle - 8.0, prev_angle + 8.0);
        VarModule::set_float(boma.object(), vars::samus::instance::AIM_ANGLE, angle);

        if angle.abs() > 45.0
        && boma.is_situation(*SITUATION_KIND_AIR) {
            if boma.is_status(*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A) {
                fighter.set_joint_rotate("shoulderr", Vector3f::new(0.0, 0.0, (angle - (45.0 * angle.signum()))));
            }
            else {
                fighter.set_joint_rotate("waist", Vector3f::new(0.0, 0.0, (angle - (45.0 * angle.signum())) * -1.0));
            }
        }
        fighter.set_joint_rotate("armr", Vector3f::new(0.0, 0.0, angle.clamp(-45.0, 45.0) * flip_y));
    }

    /* let article = if boma.is_status_one_of(&[
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A]) {
            ArticleModule::get_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE)
        }
        else {
            ArticleModule::get_article(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE)
        };

        if article != std::ptr::null_mut::<Article>() {
            println!("article if he real");
            let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            println!("object_id: {}", object_id);
            let article_boma = sv_battle_object::module_accessor(object_id);
            ModelModule::set_joint_rotate(article_boma,
                Hash40::new("top"),
                &Vector3f::new(0.0, 0.0, angle),
                MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},
                MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    */
}

unsafe fn flashshift(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER) > 0.0 {
        VarModule::sub_float(boma.object(), vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER, 1.0);

        if VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER) <= 0.0 {
            if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, 18.0, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                let lr = PostureModule::lr(boma);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, 18.0, 2, 0, 0, 0, 1.0, true);
            }
            LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
        }
    }
    if VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) > 0.0 {
        VarModule::sub_float(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER, 1.0);
        if VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) <= 0.0 {
            VarModule::set_int(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_COUNT, 0);
            VarModule::on_flag(boma.object(), vars::samus::instance::FLASHSHIFT_USED);
        }
    }
    if boma.is_status_one_of(&[
    *FIGHTER_STATUS_KIND_DEAD,
    *FIGHTER_STATUS_KIND_REBIRTH,
    *FIGHTER_STATUS_KIND_WIN,
    *FIGHTER_STATUS_KIND_LOSE,
    *FIGHTER_STATUS_KIND_ENTRY]) 
    || !boma.is_situation(*SITUATION_KIND_AIR) {
        VarModule::off_flag(boma.object(), vars::samus::instance::FLASHSHIFT_USED);
    }

    println!("flashshift main timer: {}", VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER));
    println!("flashshift chain timer: {}", VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER));
    println!("flashshift chain count: {}", VarModule::get_int(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_COUNT));
    println!("flashshift used: {}", VarModule::is_flag(boma.object(), vars::samus::instance::FLASHSHIFT_USED));
    
    if VarModule::get_float(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) > 0.0
    && VarModule::get_int(boma.object(), vars::samus::instance::FLASHSHIFT_CHAIN_COUNT) < 3 {
        // If in the middle of a flashshift chain, you can cancel any regular attack into a flashshift
        if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0
        && boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR]) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
        }
    }
}

// Shinespark charge
unsafe fn shinespark_charge(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if *FIGHTER_STATUS_KIND_RUN == status_kind && frame > 35.0 {
        if  !VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
            VarModule::on_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n04"));
        }
    }

    if VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
        /* Glow blue during speed boost
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.1};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.172, /* Green */ y: 0.439, /* Blue */ z: 0.866, /* Alpha */ w: 0.4};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
        */
        let speedboost_speed_max = ParamModule::get_float(boma.object(), ParamType::Agent, "speedboost.speed_max");
        let run_speed_mul = speedboost_speed_max / WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
        if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            lua_bind::FighterKineticEnergyMotion::set_speed_mul(boma.get_motion_energy(), run_speed_mul);
        }
        let jump_speed_mul = speedboost_speed_max / WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
        VarModule::set_float(boma.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, jump_speed_mul);
    }
    else {
        if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            lua_bind::FighterKineticEnergyMotion::set_speed_mul(boma.get_motion_energy(), 1.0);
        }
        VarModule::set_float(boma.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 1.0);
    }
}

// Shinespark Reset
unsafe fn shinespark_reset(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    let speedboost_speed_max = ParamModule::get_float(boma.object(), ParamType::Agent, "speedboost.speed_max");
    let frame = MotionModule::frame(boma);

    if !boma.is_motion_one_of(
    &[Hash40::new("attack_dash"),
    Hash40::new("special_air_lw_shinespark")]) {
        VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_USED);
    }

    println!("x speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs());
    println!("frame: {}", frame);
    // Check conditions for losing speedboost
    if VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY)
    && !([*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_WALL_JUMP].contains(&status_kind)
        || (status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE
            && frame < 6.0)
        || (status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT
            && boma.is_stick_forward())
        || ([*FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind)
            && boma.is_situation(*SITUATION_KIND_AIR)
            && (KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.8 * speedboost_speed_max
                || GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32)
                || GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32)))
        || ([*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status_kind)
            && boma.is_situation(*SITUATION_KIND_GROUND)
            && (frame <= 11.0
                || KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.8 * speedboost_speed_max))) {
        VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
        
        // If samus was in morphball, reset the status to reset the speed params to regular values
        if boma.is_motion(Hash40::new("special_lw")) || boma.is_motion(Hash40::new("special_air_lw")) {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, true);
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), frame, 1.0, 1.0);
            }
            else if boma.is_situation(*SITUATION_KIND_AIR) {
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw"), frame, 1.0, 1.0);
            }
        }
    }
    // Reset storage on death
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 0.0);
    }

    // Disable color if neither speedboost nor shinespark storage/usage are active
    /* !VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY)
    && */
    if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) == 0.0
    && !VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_USED) {
        ColorBlendModule::cancel_main_color(boma, 0);
    }
}

// Shinespark storage
unsafe fn shinespark_storage(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    // Decrement shinespark timer
    if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
        VarModule::sub_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 1.0);
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.015};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.75, /* Green */ y: 0.25, /* Blue */ z: 0.925, /* Alpha */ w: 0.6};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    }

    // Begin timer of 5 seconds and glow purple for storing shinespark with crouch
    if *FIGHTER_STATUS_KIND_SQUAT_WAIT == status_kind
    && VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
        VarModule::set_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 300.0);
        VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_READY);
        PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
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

unsafe fn shinespark_effect(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // Speedboost and shinespark stored random electric sparks
    if VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY)
    || VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
        let rng = app::sv_math::rand(hash40("fighter"), 10);
        if rng == 0 {
            let rng2 = app::sv_math::rand(hash40("fighter"), 3);
            let morphball_offset;
            if boma.is_motion_one_of(&[Hash40::new("special_lw"), Hash40::new("special_air_lw")]) {
                morphball_offset = 6.0;
            }
            else {
                morphball_offset = 0.0;
            }
            if rng2 == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 14.7 - morphball_offset, 4.3, 0, 0, 0, 0.12, true);
                if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                    LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
                }
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0 - morphball_offset, 1.0, 0, 0, 0, 0.9, true);
            }
            else if rng2 == 1 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 3.5 - morphball_offset, -6.1, 0, 0, 0, 0.09, true);
                if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                    LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
                }
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0 - morphball_offset, 1.0, 0, 0, 0, 0.9, true);
            }
            else if rng2 == 2 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 8.4 - morphball_offset, 0.2, 0, 0, 0, 0.16, true);
                if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                    LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
                }
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0 - morphball_offset, 1.0, 0, 0, 0, 0.9, true);
            }
            if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
            }
        }
    }

    // Speedboost effects
    if VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY) {
        // Speed lines
        if VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_HANDLE) == -1 {
            let handle;
            // Different speed line effects if in morphball
            if !boma.is_motion_one_of(&[Hash40::new("special_lw"), Hash40::new("special_air_lw")]) {
                handle = EffectModule::req_follow(boma, Hash40::new("sys_attack_speedline"), Hash40::new("top"), &Vector3f{x: -2.5, y: 6.5, z: 0.0}, &Vector3f{x: 0.0, y: 180.0, z: 0.0}, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
            else {
                handle = EffectModule::req_follow(boma, Hash40::new("sys_attack_speedline"), Hash40::new("top"), &Vector3f{x: -2.5, y: 4.0, z: 0.0}, &Vector3f{x: 0.0, y: 180.0, z: 0.0}, 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
            }
            EffectModule::set_rate_last(boma, 0.4);
            EffectModule::set_rgb(boma, handle, 0.2, 0.4, 10.0); // Blue
            VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_HANDLE, handle as i32);
        }
        // Jets
        if boma.status() != VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_JET_STATUS)
        && !boma.is_motion_one_of(&[Hash40::new("special_lw"), Hash40::new("special_air_lw")]) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            // EffectModule::req_follow(boma, Hash40::new("samus_jump_jet"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 15.0, y: -65.0, z: 0.0}, 1.0, true, 0, 0, 0, 0, 0, true, true);
            EffectModule::set_rate_last(boma, 0.05);
            VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_JET_STATUS, boma.status());
        }
        // Kill jets if morphball
        if VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_JET_STATUS) != -1
        && boma.is_motion_one_of(&[Hash40::new("special_lw"), Hash40::new("special_air_lw")]) {
            EffectModule::kill_kind(boma, Hash40::new("samus_jump_jet"), false, false);
            VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_JET_STATUS, -1);
        }
    }
    // Kill speedboost effects
    else if !VarModule::is_flag(boma.object(), vars::samus::instance::SHINESPARK_READY)
    && VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_HANDLE) != -1 {
        let handle = VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_HANDLE) as u32;
        EffectModule::kill(boma, handle, false, false);
        VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_HANDLE, -1);
        EffectModule::kill_kind(boma, Hash40::new("samus_jump_jet"), false, false);
        VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_EFFECT_JET_STATUS, -1);
    }

    // Shinespark stored purple aura
    if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0
    && VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE) == -1 {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0); // Purple effects
        VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE, handle as i32);
    }
    // Kill purple aura
    else if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) == 0.0
    && VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE) != -1 {
        let handle = VarModule::get_int(boma.object(), vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE) as u32;
        EffectModule::kill(boma, handle, false, false);
        VarModule::set_int(boma.object(), vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE, -1);
    }
}

// Morph Ball Crawl
// PUBLIC
pub unsafe fn morphball_crawl(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if boma.is_motion_one_of(&
    [Hash40::new("special_lw"),
    Hash40::new("special_air_lw")]) {
        // Freeze motion rate if x speed is 0 so that ball doesn't roll if you're standing still
        if 20.0 <= frame
        && frame < 40.0 
        && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0 {
            MotionModule::set_rate(boma, 0.0);
        }
        else {
            MotionModule::set_rate(boma, 1.0);
        }
        // Place bomb by pressing Attack
        if boma.is_button_trigger(Buttons::AttackAll)
        && frame < 40.0
        && VarModule::get_int(boma.object(), vars::samus::instance::BOMB_COUNTER) < 8 {
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
            ArticleModule::shoot_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            VarModule::inc_int(boma.object(), vars::samus::instance::BOMB_COUNTER);
        }
        // Exit morphball by pressing Special
        if boma.is_button_trigger(Buttons::SpecialAll)
        && 20.0 <= frame
        && frame < 40.0 {
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, true);
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 40.0, 1.0, 1.0);
            }
            else if boma.is_situation(*SITUATION_KIND_AIR) {
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw"), 40.0, 1.0, 1.0);
            }
            // Have samus oriented in the direction of the stick because the orientation from before morphball was activated may be unintuitive
            if (boma.stick_x() != 0.0) {
                PostureModule::set_lr(boma, boma.stick_x().signum());
                PostureModule::update_rot_y_lr(boma);
            }
        }
        // Stay in morphball after a bomb jump
        if frame >= 12.0
        && [*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
        *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A].contains(&status_kind) {
                if boma.is_situation(*SITUATION_KIND_GROUND) {
                    StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, false);
                }
                else if boma.is_situation(*SITUATION_KIND_AIR) {
                    StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, false);
                }
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 20.0, 1.0, 1.0);
        }
        // Loop before end of morphball
        else if 38.0 <= frame
        && frame < 40.0 {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 20.0, 1.0, 1.0);
        }
        // Allow jumping and double jumping in morphball
        if boma.is_input_jump()
        && 11.0 < frame
        && frame < 40.0 {
            let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
            let mini_jump_y = WorkModule::get_param_float(boma, hash40("mini_jump_y"), 0);
            let jump_speed = Vector3f{x: 0.0, y: (air_accel_y * (mini_jump_y / (0.5 * air_accel_y)).sqrt()), z: 0.0};

            if boma.is_situation(*SITUATION_KIND_GROUND) {
                StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                KineticModule::add_speed(boma, &jump_speed);
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
                PLAY_SE(fighter, Hash40::new("se_samus_jump03"));
                EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);

            }
            else if boma.is_situation(*SITUATION_KIND_AIR)
            && boma.get_num_used_jumps() < boma.get_jump_count_max() {
                let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
                KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticModule::add_speed(boma, &jump_speed);
                WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                PLAY_SE(fighter, Hash40::new("se_samus_jump03"));
                EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        // Ballspark by pressing Shield
        if VarModule::get_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER) > 0.0
        && boma.is_button_trigger(Buttons::Guard)
        && 11.0 < frame
        && frame < 40.0 {
            VarModule::on_flag(boma.object(), vars::samus::instance::SHINESPARK_USED);
            VarModule::set_float(boma.object(), vars::samus::instance::SHINESPARK_TIMER, 0.0);
            if boma.is_situation(*SITUATION_KIND_GROUND) {
                StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                let hop_speed = Vector3f{x: 0.0, y: 0.45, z: 0.0};
                KineticModule::add_speed(boma, &hop_speed);
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw_shinespark"), 18.0, 1.0, 1.0);
            }
            else if boma.is_situation(*SITUATION_KIND_AIR) {
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw_shinespark"), 18.0, 1.0, 1.0);
            }
        }
    }

    // Reset bomb counter
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
        morphball_crawl(fighter, &mut *info.boma, info.status_kind, info.frame);
        nspecial_cancels(&mut *info.boma, info.status_kind, info.situation_kind);
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    change_missiles(fighter, boma);
    aim_gun(fighter, boma);
    flashshift(fighter, boma);
    shinespark_charge(fighter, boma, id, status_kind, frame);
    shinespark_reset(boma, id, status_kind);
    shinespark_storage(fighter, boma, id, status_kind);
    shinespark_air(boma);
    shinespark_effect(fighter, boma);
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
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}