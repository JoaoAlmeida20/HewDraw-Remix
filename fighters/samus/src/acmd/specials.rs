
use super::*;

#[acmd_script( agent = "samus", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn special_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 92, 100, 180, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 98, 100, 180, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 92, 100, 80, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 98, 100, 80, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 84, 100, 100, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 100, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 84, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 82, 100, 40, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 82, 100, 20, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 90, 100, 20, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 200, 0, 56, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "samus", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 93, 100, 115, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 96, 100, 115, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 93, 100, 60, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 96, 100, 60, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 96, 100, 70, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 98, 100, 70, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 96, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.0, 98, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.6, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13135c5462), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 250, 0, 50, 10.0, 0.0, 6.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUS_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "samus", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_sphere").hash as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
}

#[acmd_script( agent = "samus", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_sphere").hash as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        VarModule::on_flag(boma.object(), vars::common::instance::SPECIAL_STALL_USED);
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
    }
}

#[acmd_script( agent = "samus", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT, low_priority )]
unsafe fn special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_bomb_jump"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.48, true);
    }
    
}

#[acmd_script( agent = "samus", script = "game_special" , category = ACMD_GAME , low_priority)]
unsafe fn special(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "samus", script = "effect_special" , category = ACMD_EFFECT , low_priority)]
unsafe fn special_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EffectModule::req_follow(boma, Hash40::new("samus_cshot_hold"), Hash40::new("top"), &Vector3f{x: 0.0, y: 10.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.8, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("samus_cshot_hold"), false, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_attack_speedline"), Hash40::new("top"), &Vector3f{x: -5.0, y: 6.5, z: 0.0}, &Vector3f{x: 0.0, y: 180.0, z: 0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rate_last(boma, 0.4);
        EffectModule::set_rgb(boma, handle, 0.2, 0.4, 10.0); // Blue
    }
}

#[acmd_script( agent = "samus", script = "sound_special" , category = ACMD_SOUND , low_priority)]
unsafe fn special_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n02"));
        SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_samus_special_n02"), 0.75);
    }
}

#[acmd_script( agent = "samus", script = "expression_special" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn special_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "samus", script = "game_specialairlw_shinespark" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_lw_shinespark(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let speed = 8.0;
    let mut x_mul = 0.0;
    let mut y_mul = 0.0;

    frame(lua_state, 20.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::clear_speed_all(boma);
        MotionModule::set_rate(boma, 0.1);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        if boma.stick_x().abs() > 0.93 {
            x_mul = boma.stick_x().signum();
            y_mul = 0.0; 
        }
        else if boma.stick_y().abs() > 0.93 {
            x_mul = 0.0;
            y_mul = boma.stick_y().signum();
        }
        else {
            x_mul = boma.stick_x().signum();
            y_mul = boma.stick_y().signum();
        }
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy::set_accel(lua_state);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy::set_accel_x_add(lua_state);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy::set_accel_x_mul(lua_state);
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed, speed);
        sv_kinetic_energy::set_limit_speed(lua_state);
        let addSpeed = Vector3f { x: speed * x_mul * PostureModule::lr(boma), y: speed * y_mul, z: 0.0 };
        KineticModule::add_speed(boma, &addSpeed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 120, 0, 80, 3.8, 0.0, 3.0, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        MotionModule::set_rate(boma, 1.6);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        let subSpeed = Vector3f { x: -speed * x_mul * PostureModule::lr(boma), y: -speed * y_mul, z: 0.0 };
        KineticModule::add_speed(boma, &subSpeed);
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 0.1);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        VarModule::off_flag(boma.object(), vars::samus::instance::SHINESPARK_USED);
        StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, false);
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 20.0, 1.0, 1.0);
    }
}

#[acmd_script( agent = "samus", script = "sound_specialairlw_shinespark" , category = ACMD_SOUND , low_priority)]
unsafe fn special_air_lw_shinespark_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13eb2847e0));
        PLAY_SE(fighter, Hash40::new_raw(0x0e29ee1d3f));
        PLAY_SE_REMAIN(fighter, Hash40::new_raw(0x14614c32aa));
    }
}

#[acmd_script( agent = "samus", script = "effect_specialairlw_shinespark" , category = ACMD_EFFECT , low_priority)]
unsafe fn special_air_lw_shinespark_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0 , 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0 , 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0 , 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        special_hi,
        special_air_hi,
        special_lw,
        special_air_lw,
        special_lw_effect,
        special,
        special_effect,
        special_sound,
        special_expression,
        special_air_lw_shinespark,
        special_air_lw_shinespark_sound,
        special_air_lw_shinespark_effect
    );
}

