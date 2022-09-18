
use super::*;

#[acmd_script( agent = "samus", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn attack_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let mut ySpeed = 0.0;
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            VarModule::off_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_READY);
            VarModule::set_float(fighter.battle_object, vars::samus::instance::SHINESPARK_TIMER, 0.0);
            VarModule::on_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED);
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::clear_speed_all(boma);
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::get_float(fighter.battle_object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
            VarModule::off_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_READY);
            VarModule::set_float(fighter.battle_object, vars::samus::instance::SHINESPARK_TIMER, 0.0);
            VarModule::on_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED);
            MotionModule::set_rate(boma, 0.25);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 55, 114, 0, 70, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 8.5, 55, 114, 0, 70, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("bust"), 8.5, 55, 114, 0, 70, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED) {
            MotionModule::set_rate(boma, 0.5);
            if boma.is_situation(*SITUATION_KIND_AIR) {
                if boma.stick_y().abs() > 0.38 {
                    ySpeed = 2.4 * boma.stick_y().signum();
                }
                /* WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                SET_SPEED_EX(fighter, 2.4, ySpeed, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); */
                let addSpeed = Vector3f { x: 2.4, y: ySpeed, z: 0.0 };
                KineticModule::add_speed(boma, &addSpeed);
            }
            else {
                /* WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                SET_SPEED_EX(fighter, 1.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK); */
                let addSpeed = Vector3f { x: 1.6, y: 0.0, z: 0.0 };
                KineticModule::add_speed(boma, &addSpeed);
            }
            ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 110, 0, 70, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 16.0, 361, 110, 0, 70, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("bust"), 16.0, 361, 110, 0, 70, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 0.2);
        }
                 }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 55, 114, 0, 70, 5.3, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(4.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 10.0, 55, 114, 0, 70, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("bust"), 10.0, 55, 114, 0, 70, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 115, 0, 75, 5.3, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(4.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 13.0, 361, 115, 0, 75, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("bust"), 13.0, 361, 115, 0, 75, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 0.2);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 60, 0, 78, 4.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 6.0, 80, 60, 0, 78, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("bust"), 6.0, 80, 60, 0, 78, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 120, 0, 80, 4.0, 0.0, 9.0, 6.0, Some(0.0), Some(9.0), Some(5.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 10.0, 361, 120, 0, 80, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("bust"), 10.0, 361, 120, 0, 80, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(9.0), Some(2.0), 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
            ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter, 0, 1, 2, 0.2);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            AttackModule::clear_all(boma);
            MotionModule::set_rate(boma, 0.1);
            let subSpeed1 = Vector3f { x: -2.4, y: -ySpeed, z: 0.0 };
            KineticModule::add_speed(boma, &subSpeed1);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        if boma.is_situation(*SITUATION_KIND_AIR) {
            MotionModule::set_frame(boma, MotionModule::end_frame(boma), true);
        }
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED) {
            let subSpeed1 = Vector3f { x: -1.6, y: 0.0, z: 0.0 };
            KineticModule::add_speed(boma, &subSpeed1);
            MotionModule::set_rate(boma, 0.25);
        }
    }
	wait(lua_state, 10.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED) {
            MotionModule::set_rate(boma, 1.0);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
}

#[acmd_script( agent = "samus", script = "sound_attackdash" , category = ACMD_SOUND , low_priority)]
unsafe fn attack_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13eb2847e0));
        if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED)  {
            PLAY_SE(fighter, Hash40::new_raw(0x0e29ee1d3f));
            PLAY_SE_REMAIN(fighter, Hash40::new_raw(0x14614c32aa));
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x14fbe97afc));
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x15eb15be2a));
    }
    
}

#[acmd_script( agent = "samus", script = "effect_attackdash" , category = ACMD_EFFECT , low_priority)]
unsafe fn attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);    
    }
    frame(lua_state, 9.0);
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::SHINESPARK_USED) {
        let num_loops = match boma.is_situation(*SITUATION_KIND_GROUND) {
            true => 1,
            false => 2,
        };

        for _ in 0..num_loops {
            if is_excute(fighter) {
                BURN_COLOR(fighter, 0.699999988, 0.200000003, 1.0, 0.699999988);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 14.7, 4.3, 0, 0, 0, 0.23, true);
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                BURN_COLOR_FRAME(fighter, 1, 0.699999988, 0.200000003, 1.0, 0);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                BURN_COLOR_NORMAL(fighter);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 3.5, -6.1, 0, 0, 0, 0.17, true);
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                FLASH(fighter, 1, 0.699999988, 1.0, 0.5);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                FLASH_FRM(fighter, 1, 1, 0.699999988, 1.0, 0);
                COL_NORMAL(fighter);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 8.4, 0.2, 0, 0, 0, 0.32, true);
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                BURN_COLOR(fighter, 0.699999988, 0.200000003, 1.0, 0.699999988);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
                LAST_EFFECT_SET_RATE(fighter, 1);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                BURN_COLOR_FRAME(fighter, 1, 0.699999988, 0.200000003, 1.0, 0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 6.2, 5.6, 0, 0, 0, 0.2, true);
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0, 1.0, 0, 0, 0, 1.8, true);
            }
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                BURN_COLOR_NORMAL(fighter);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_elec_s"), false, true);
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
            }
            wait(lua_state, 1.0);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "samus", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn attack_11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 70, 40, 0, 35, 4.0, 0.0, 10.0, 9.0, Some(0.0), Some(10.0), Some(16.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 361, 10, 0, 25, 2.5, 0.0, 3.5, 9.0, Some(0.0), Some(3.5), Some(16.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.5, 70, 0, 40, 35, 2.5, 0.0, 3.5, 9.0, Some(0.0), Some(3.5), Some(16.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 1.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 1.0, false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        attack_dash,
        attack_dash_sound,
        attack_dash_effect,
        attack_11,
    );
}

