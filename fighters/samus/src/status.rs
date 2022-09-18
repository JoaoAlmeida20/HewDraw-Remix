use super::*;
use globals::*;
 

pub fn install() {
    install_status_scripts!(
        pre_attack_lw3,
        main_attack_lw3,
        pre_special_s,
        main_special_s,
        end_special_s,
        exit_special_n,
        samus_cshot_shoot_init,
        samus_supermissile_ready_main,
        samus_supermissile_straight_pre,
        samus_supermissile_straight_main,
    );
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_attack_lw3(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32, // originally *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_attack_lw3(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_attack_lw3_loop as *const () as _))
}

unsafe extern "C" fn main_attack_lw3_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if situation != *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(boma) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }

        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
            if StatusModule::is_changing(boma)
            || (ComboModule::count(boma) < WorkModule::get_param_int(boma, hash40("s3_combo_max"), 0) as u64
                && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
                fighter.attack_s3_mtrans();
            }
        }

        if fighter.global_table[CURRENT_FRAME].get_f32() >= 7.0 {
            if fighter.global_table[globals::PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW.into(), false.into());
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 6.0, 1.0, 1.0);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            fighter.sub_transition_group_check_ground_jump_mini_attack();
            fighter.sub_transition_group_check_ground_jump();
        }

        let mut motion_mul = 1.0;
        
        if fighter.global_table[CURRENT_FRAME].get_f32() >= 8.0
        && fighter.global_table[CURRENT_FRAME].get_f32() <= 20.0 {
            motion_mul *= 1.8;
            MotionModule::set_rate(boma, 0.7);
        }
        else {
            MotionModule::set_rate(boma, 1.0);
        }

        if VarModule::is_flag(fighter.object(), vars::samus::instance::SHINESPARK_READY) {
            motion_mul *= 1.7;
            if fighter.global_table[CURRENT_FRAME].get_f32() >= 20.0
            && fighter.global_table[STICK_X].get_f32() * PostureModule::lr(boma) > 0.75 {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), false.into());
            }
        }

        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            motion_mul
        );

        let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        if 0 < jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor)
            && fighter.sub_check_button_jump().get_bool() {
                let log = fighter.status_attack();
                let info = log[0x10f40d7b92u64].get_i64();
                let mot = MotionModule::motion_kind(fighter.module_accessor);
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EXPRESSION,
                    Hash40::new_raw(mot),
                    -1
                );
                WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
        if 1 == jump_attack_frame {
            if fighter.global_table[IS_STOPPING].get_bool()
            && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
                let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    PostureModule::set_stick_lr(boma, 0.0);
    PostureModule::update_rot_y_lr(boma);
    MotionModule::change_motion(boma, Hash40::new("special"), 0.0, 1.0, false, 0.0, false, false);

    // Prevent previous flashshift's chain timer from running out before new timer starts at end of this flashshift
    VarModule::set_float((*boma).object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER,999.0);
    VarModule::inc_int((*boma).object(), vars::samus::instance::FLASHSHIFT_CHAIN_COUNT);

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(main_special_s_loop as *const () as _))
}

unsafe extern "C" fn main_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }

    if situation == *SITUATION_KIND_AIR {
        let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let is_turn_dash = compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH);
        let is_jump = compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP);
        if (touch_right || touch_left) && (is_turn_dash || is_jump) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if situation == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn end_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    VarModule::set_float((*boma).object(), vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER, 180.0);
    VarModule::set_float((*boma).object(), vars::samus::instance::FLASHSHIFT_CHAIN_TIMER, 25.0);

    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn exit_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Prevents losing charge if you switch to missiles during neutral special startup
    if fighter.global_table[STATUS_KIND] == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G
    || fighter.global_table[STATUS_KIND] == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        return 0.into();
    }
    original!(fighter)
}

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn samus_cshot_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let obattle_object = (*oboma).object();
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED)
    && !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            1.0,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    // let angle = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("angle")) as f32; piece of shit an angle in radians as an integer wtf is that
    let angle = if (*oboma).is_situation(*SITUATION_KIND_AIR) {
        VarModule::get_float(obattle_object, vars::samus::instance::AIM_ANGLE)
    }
    else {
        VarModule::get_float(obattle_object, vars::samus::instance::AIM_ANGLE).clamp(-45.0, 45.0)
    };
    let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_speed"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_speed"));
    let speed = (max_speed - min_speed) * charge + min_speed;
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
        let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));
        let scale = (max_scale - min_scale) * charge + min_scale;
        if (0.3..1.0).contains(&scale) {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samus_cshot_bullet_sub_a"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
        }
        else {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samus_cshot_bullet_sub_b"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
        }
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet_sub"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samus_supermissile_ready_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = (*oboma).object();
    let lr = PostureModule::lr(boma);
    let accel_frame = WorkModule::get_param_int(boma, hash40("param_supermissile"), hash40("s_acc_f"));
    let x_speed_start = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_spd_x0"));
    let y_speed_start = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_spd_y0"));
    let rot = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_rot"));

    let angle = if (*oboma).is_situation(*SITUATION_KIND_AIR) {
        VarModule::get_float(oobject, vars::samus::instance::AIM_ANGLE)
    }
    else {
        VarModule::get_float(oobject, vars::samus::instance::AIM_ANGLE).clamp(-45.0, 45.0)
    };
    VarModule::set_float(oobject, vars::samus::instance::PROJECTILE_ANGLE, angle);

    MotionModule::change_motion(boma, Hash40::new("ready"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int(boma, accel_frame, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        x_speed_start * lr,
        -y_speed_start
    );

    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -(x_speed_start * lr) / (accel_frame as f32),
        y_speed_start / (accel_frame as f32)
    );

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        -(rot / (accel_frame as f32)),
        0.0,
        0.0
    );

    weapon.set_joint_rotate("rot", Vector3f::new(-angle, 0.0, 0.0));

    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(samus_supermissile_ready_main_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(samus_supermissile_ready_main_loop as *const () as _))
    
}

unsafe extern "C" fn samus_supermissile_ready_main_substatus(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    let boma = weapon.module_accessor;

    if param_2.get_bool() {
        WorkModule::dec_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME)
    }
    0.into()
}

unsafe extern "C" fn samus_supermissile_ready_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = (*oboma).object();

    let lr = PostureModule::lr(boma);
    let angle = VarModule::get_float(oobject, vars::samus::instance::PROJECTILE_ANGLE);
    weapon.set_joint_rotate("rot", Vector3f::new(-angle, 0.0, 0.0));

    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }
    else if WorkModule::get_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME) <= 0 {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn samus_supermissile_straight_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL, // Originally _NONE
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samus_supermissile_straight_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = (*oboma).object();

    let angle = VarModule::get_float(oobject, vars::samus::instance::PROJECTILE_ANGLE);
    let accel = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_acc_x"));
    let max_speed = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_spd_x_max"));
    let lr = PostureModule::lr(boma);
    let accel_x = angle.to_radians().cos() * accel * lr;
    let accel_y = angle.to_radians().sin() * accel;
    let max_speed_x = angle.to_radians().cos() * max_speed;
    let max_speed_y = angle.to_radians().sin() * max_speed;

    weapon.set_joint_rotate("rot", Vector3f::new(-angle, 0.0, 0.0));

    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        accel_x,
        accel_y
    );

    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        max_speed_x,
        max_speed_y
    );

    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        max_speed_x,
        max_speed_y
    );

    MotionModule::change_motion(boma, Hash40::new("straight"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(samus_supermissile_straight_main_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(samus_supermissile_straight_main_loop as *const () as _))
}

unsafe extern "C" fn samus_supermissile_straight_main_substatus(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    let boma = weapon.module_accessor;

    if param_2.get_bool() {
        WorkModule::dec_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME)
    }
    0.into()
}

unsafe extern "C" fn samus_supermissile_straight_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = (*oboma).object();

    let lr = PostureModule::lr(boma);
    let angle = VarModule::get_float(oobject, vars::samus::instance::PROJECTILE_ANGLE);
    weapon.set_joint_rotate("rot", Vector3f::new(-angle, 0.0, 0.0));

    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_ALL as u32)
    || WorkModule::get_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME) <= 0 {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }
    0.into()
}