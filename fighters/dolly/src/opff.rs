// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff, backdash_energy});
use super::*;
use globals::*;

 
unsafe fn dtilt_repeat_increment(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64) {
    if motion_kind == hash40("attack_lw3")
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        &&  !VarModule::is_flag(boma.object(), vars::shotos::REPEAT_INCREMENTED) {
        //VarModule::inc_int(boma.object(), vars::common::REPEAT_NUM_LW);
        VarModule::on_flag(boma.object(), vars::shotos::REPEAT_INCREMENTED);
    }
}

// Terry Power Wave Dash Cancel and Super Cancels
unsafe fn power_wave_dash_cancel_super_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat: [i32; 4], motion_kind: u64, frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    let prev_situation_kind = StatusModule::prev_situation_kind(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        // Super Cancel
        if frame > 21.0 {
            // Check to see if supers are available
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Enable transition term
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);

                // Buster Wolf
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, false);
                }

                // Power Geyser
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, false);
                }
            }
        }

        // Triple Geyser
        if MeterModule::level(boma.object()) >= 10 {
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                
            }
        }

        // Dash Cancel
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_S3 {
            if frame > 36.0 {
                if situation_kind == *SITUATION_KIND_GROUND {
                    if boma.is_cat_flag(Cat1::Dash) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                    }
                }
            }
        } else {
            if frame > 33.0 {
                if situation_kind == *SITUATION_KIND_GROUND {
                    if boma.is_cat_flag(Cat1::Dash) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
                    }
                }
            }
        }
    }
}

// Special and Super Cancels into Triple Geyser
unsafe fn special_super_cancels_triple_geyser(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat4: i32, motion_kind: u64) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if [*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind) {
        // Triple Geyser
        if MeterModule::level(boma.object()) >= 10 {
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
            }
        }
    }

    // Super Cancels into Triple Geyser
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind)
        && motion_kind == 0x13434c5490 as u64 {
        if MeterModule::level(boma.object()) >= 6 {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
            
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, true);
            }
        }
    }
}

// Terry Burn Knuckle Land Cancel
// Check for aerial startup
unsafe fn burn_knuckle_land_cancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if motion_kind == hash40("special_air_f_start") {
        if situation_kind == *SITUATION_KIND_AIR {
            VarModule::on_flag(boma.object(), vars::common::AIR_SPECIAL_USED);
        }
    }
    if VarModule::is_flag(boma.object(), vars::common::AIR_SPECIAL_USED) {
        if [*FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END,
            *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK].contains(&status_kind) {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
            }
        }
    }
}

// Terrry Super Special Meter Activation
unsafe fn super_special_meter_activation(boma: &mut BattleObjectModuleAccessor) {
    if MeterModule::level(boma.object()) >= 4 {
        WorkModule::on_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
    }
    if MeterModule::level(boma.object()) < 4 {
        WorkModule::off_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
    }
}

// Cancel supers early
unsafe fn cancel_supers_early(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2].contains(&status_kind) {
        if frame < 25.0 {
            if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, false);
                }
            }
        }
    }
}

// Super Cancels
unsafe fn super_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, cat4: i32, motion_kind: u64) {
    let mut agent_base = fighter.fighter_base.agent_base;
    // Power Geyser
    if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL {
        if MeterModule::level(boma.object()) >= 2 {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            // Buster Wolf
            if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                    | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                
                VarModule::on_flag(boma.object(), vars::common::SUPER_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, false);
            }
        }
    }
    // Buster Wolf
    if [*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind)
        || motion_kind == 0x13434c5490 as u64 {
        if MeterModule::level(boma.object()) >= 2 {
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
            // Power Geyser
            if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                    | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                VarModule::on_flag(boma.object(), vars::common::SUPER_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, false);
            }
        }
    }
}

// Turn off Super Cancel Flag
unsafe fn super_cancel_flag_off(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if ![*FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,
        *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::common::SUPER_CANCEL);
    }
}

// Terry Shield Stop and Run Drop
unsafe fn shield_stop_run_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_y: f32, situation_kind: i32) {
    if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_GUARD_TRIGGER)
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH)
        && situation_kind == *SITUATION_KIND_GROUND
        && [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK].contains(&status_kind)
    {
        let flick_y_sens = ParamModule::get_float(boma.object(), ParamType::Common, "general_flick_y_sens");
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_GUARD_ON, true);
        ControlModule::clear_command(boma, true);
        if GroundModule::is_passable_ground(boma) && boma.is_flick_y(flick_y_sens) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
        }
    }
}

// TRAINING MODE
// Full Meter Gain via shield during taunt
unsafe fn full_meter_training_taunt(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                let meter_max = ParamModule::get_float(boma.object(), ParamType::Common, "meter_max_damage");
                MeterModule::add(boma.object(), meter_max);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //dtilt_repeat_increment(boma, id, motion_kind); // UNUSED
    power_wave_dash_cancel_super_cancels(fighter, boma, id, status_kind, situation_kind, cat, motion_kind, frame);
    special_super_cancels_triple_geyser(fighter, boma, id, status_kind, cat[3], motion_kind);
    //burn_knuckle_land_cancel(boma, id, status_kind, situation_kind, motion_kind); // UNUSED
    super_special_meter_activation(boma);
    cancel_supers_early(boma, status_kind, situation_kind, frame);
    super_cancels(fighter, boma, id, status_kind, cat[3], motion_kind);
    super_cancel_flag_off(boma, id, status_kind);
    shield_stop_run_drop(boma, status_kind, stick_y, situation_kind);
    full_meter_training_taunt(fighter, boma, status_kind);

    // Magic Series
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    common::opff::backdash_energy(fighter);
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    let cat1 = cat[0];
    let cat4 = cat[3];
    // Terry
    // Level 1: Jab Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for tilt attack inputs
            if motion_kind != hash40("attack_13") {
                if boma.is_cat_flag(Cat1::AttackS3) {
                    VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
                }
            }

            // Check for smash attack inputs
            if boma.is_cat_flag(Cat1::AttackS4) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackHi4) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackLw4) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for HCF inputs for Power Charge
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

        }

    }


    // Level 2: Tilt Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for utilt inputs during ftilt
            if motion_kind == hash40("attack_s3_s") {
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    //VarModule::on_flag(boma.object(), vars::common::TILT_CHECKS);
                    VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
            }

            // Check for smash attack inputs
            if boma.is_cat_flag(Cat1::AttackS4) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackHi4) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
            }
            if boma.is_cat_flag(Cat1::AttackLw4) {
                VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
            }

            // Check for HCF inputs for Power Charge
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

            // Check for jump and dash inputs during utilt, and dash inputs during ftilt
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && frame > 13.0 {
                if boma.is_input_jump() {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT,true);
                }
                if boma.is_cat_flag(Cat1::Dash) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH,false);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
                && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && frame > 13.0 {
                if boma.is_cat_flag(Cat1::Dash) &&  !VarModule::is_flag(boma.object(), vars::common::MAGIC_CANCEL_ADDITIONAL) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH,false);
                }
            }
        }
    }

    // Dash Attack Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            // Cancel into supers
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            }

            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Buster Wolf
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,false);
                }

                // Power Geyser
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,false);
                }
            }
        }
    }


    // Smash Cancels
    if [*FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {

            // Check for HCF inputs for Power Charge
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

            // Check for special attack inputs

            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
            WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            }

            // Power Wave
            if boma.is_cat_flag(Cat1::SpecialN) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
            }
            // Burn Knuckle
            if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_forward() {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
            }
            if boma.is_cat_flag( Cat4::SpecialNCommand) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,false);
            }
            // Crack Shoot
            if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_backward() {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,false);
            }
            if boma.is_cat_flag( Cat4::SpecialSCommand) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,false);
            }
            // Rising Tackle
            if boma.is_cat_flag(Cat1::SpecialHi) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
            }
            if boma.is_cat_flag( Cat4::SpecialHi2Command) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,false);
            }
            // Power Dunk
            if boma.is_cat_flag(Cat1::SpecialLw) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
            }
            if boma.is_cat_flag( Cat4::SpecialHiCommand) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,false);
            }

            // Check for HCF inputs for Power Charge
            if boma.is_cat_flag( Cat4::SpecialN2Command) {
                ControlModule::clear_command(boma, true);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH,false);
            }

            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Buster Wolf
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,false);
                }

                // Power Geyser
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,false);
                }
            }
        }
    }

    // Aerial Cancels
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            // Aerial Magic Series
            // Nair
            if motion_kind == hash40("attack_air_n") {
                /*
                   if (boma.is_cat_flag(Cat1::AttackAirN) || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N))) {
                   StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                   }
                   */
                //if (boma.is_cat_flag(Cat1::AttackAirF) ||
                if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_forward())
                    || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_forward()) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (boma.is_cat_flag(Cat1::AttackAirB) ||
                if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_backward())
                    || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_backward()) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                    //PostureModule::reverse_lr(boma);
                }
                //if (boma.is_cat_flag(Cat1::AttackAirHi) ||
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (boma.is_cat_flag(Cat1::AttackAirLw) ||
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
            // Fair
            if motion_kind == hash40("attack_air_f") {
                //if (boma.is_cat_flag(Cat1::AttackAirB) ||
                if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_backward())
                    || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_backward()) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                    //PostureModule::reverse_lr(boma);
                }
                //if (boma.is_cat_flag(Cat1::AttackAirHi) ||
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
                //if (boma.is_cat_flag(Cat1::AttackAirLw) ||
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
            // Bair
            if motion_kind == hash40("attack_air_b") {

            }
            // Uair
            if motion_kind == hash40("attack_air_hi") {
                //if (boma.is_cat_flag(Cat1::AttackAirB) ||
                if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_backward())
                    || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_backward()) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                    //PostureModule::reverse_lr(boma);
                }
                //if (boma.is_cat_flag(Cat1::AttackAirLw) ||
                if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                                        | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
            // Dair
            if motion_kind == hash40("attack_air_lw") {
                //if (boma.is_cat_flag(Cat1::AttackAirB) ||
                if (boma.is_cat_flag(Cat1::AttackS3) && boma.is_stick_backward())
                    || (boma.is_cat_flag(Cat1::AttackS4) && boma.is_stick_backward()) {
                    //VarModule::on_flag(boma.object(), vars::shotos::IS_MAGIC_SERIES_CANCEL);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR,false);
                }
            }
        }
    }

    // Special Cancels
    if [*FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK].contains(&status_kind) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {

            // Super Cancels for all specials
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
            }
            if WorkModule::is_flag(boma, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
                // Buster Wolf
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL,false);
                }

                // Power Geyser
                if compare_mask(cat4, *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND
                                        | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2,false);
                }
            }

            // Burn Knuckle cancels
            if [*FIGHTER_STATUS_KIND_SPECIAL_S,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK].contains(&status_kind) {

                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

                // Each cancel costs 2 meter
                if MeterModule::level(boma.object()) >= 1 {
                    // Crack Shoot
                    if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_backward() {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,false);
                        
                    }
                    if boma.is_cat_flag( Cat4::SpecialSCommand) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,false);
                        
                    }
                    // Rising Tackle
                    if boma.is_cat_flag(Cat1::SpecialHi) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                        
                    }
                    if boma.is_cat_flag( Cat4::SpecialHi2Command) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,false);
                        
                    }
                    // Power Dunk
                    if boma.is_cat_flag(Cat1::SpecialLw) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                        
                    }
                    if boma.is_cat_flag( Cat4::SpecialHiCommand) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,false);
                        
                    }
                }
            }

            // Rising Tackle cancels
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND,
                *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {

                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);

                // Each cancel costs 2 meter
                if MeterModule::level(boma.object()) >= 1 {
                    // Power Wave
                    if boma.is_cat_flag(Cat1::SpecialN) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                        
                    }
                    // Burn Knuckle
                    if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_forward() {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                        
                    }
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,false);
                        
                    }
                    // Crack Shoot
                    if boma.is_cat_flag(Cat1::SpecialS) && boma.is_stick_backward() {
                        
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,false);
                        
                    }
                    if boma.is_cat_flag( Cat4::SpecialSCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,false);
                        
                    }
                    // Power Dunk
                    if boma.is_cat_flag(Cat1::SpecialLw) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                        
                    }
                    if boma.is_cat_flag( Cat4::SpecialHiCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,false);
                        
                    }
                }
            }
        }
    }

    // Super Cancels
    /*
       if (status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2{
           if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){

           }
       }
       */
}
#[utils::macros::opff(FIGHTER_KIND_DOLLY )]
pub fn dolly_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        MeterModule::update(fighter.object(), true);
        if fighter.is_button_on(Buttons::AppealAll) {
            MeterModule::show(fighter.object());
        } else {
            MeterModule::stop_show(fighter.object());
        }
		dolly_frame(fighter)
    }
}

pub unsafe fn dolly_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
