
use super::*;

#[acmd_script( agent = "samus", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_samus_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_samus_step_right_m"));
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_samus_step_left_m"))
    }
}

#[acmd_script( agent = "samus", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn samus_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "samus_supermissile", script = "game_ready" , category = ACMD_GAME , low_priority)]
unsafe fn samus_supermissile_ready_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(boma);
	}
    
}

#[acmd_script( agent = "samus_supermissile", script = "game_straight" , category = ACMD_GAME , low_priority)]
unsafe fn samus_supermissile_straight_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = (*oboma).object();
	if is_excute(fighter) {
        let angle = VarModule::get_float(oobject, vars::samus::instance::PROJECTILE_ANGLE);
        // funny downward missile spike
        if angle < -70.0 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
    }
    
}

#[acmd_script( agent = "samus_supermissile", script = "effect_straight" , category = ACMD_EFFECT , low_priority)]
unsafe fn samus_supermissile_straight_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = (*oboma).object();
	if is_excute(fighter) {
        let angle = VarModule::get_float(oobject, vars::samus::instance::PROJECTILE_ANGLE);
		EFFECT_FOLLOW(fighter, Hash40::new("samus_missile_straight"), Hash40::new("top"), 0, 0, -1, -angle, 0, 0, 1, true);
    }
    
}

#[acmd_script( agent = "samus_supermissile", script = "game_sburst" , category = ACMD_GAME , low_priority)]
unsafe fn samus_supermissile_sburst_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 30, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }

}

#[acmd_script( agent = "samus_cshot", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn samus_cshot_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 25.0, 40, 64, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
	}
    
}

#[acmd_script( agent = "samus", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "samus", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "samus", scripts = [ "effect_jumpfront", "effect_jumpfrontmini" ] , category = ACMD_EFFECT , low_priority)]
unsafe fn jump_front_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        dash_sound,
        samus_turn_dash_game,
        samus_supermissile_ready_game,
        samus_supermissile_straight_game,
        samus_supermissile_straight_effect,
        samus_supermissile_sburst_game,
        samus_cshot_shoot_game,
        escape_air_game,
        escape_air_slide_game,
        jump_front_effect,
    );
}

