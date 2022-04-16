// opff import
utils::import_noreturn!(common::opff::{fighter_common_opff, check_b_reverse});
use super::*;
use globals::*;

 
unsafe fn winged_pikmin_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, cat1: i32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT].contains(&status_kind) {
        if boma.is_cat_flag(Cat1::SpecialN) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, false);
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
        }
    }
}

// Olimar Pikmin Order B-Reverse
unsafe fn pikmin_order_b_reverse(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        common::opff::check_b_reverse(fighter);
    }
}

pub unsafe fn solimar_scaling(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    let pikmin_count = WorkModule::get_int(boma, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if pikmin_count == 0 {
        let olimar_hand_scale = Vector3f{x: 1.5, y: 1.35, z: 1.35};
        let olimar_hand_midpoint_scale = Vector3f{x: 1.2, y: 1.17, z: 1.17};
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if frame > 4.0 && frame < 15.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &olimar_hand_scale);
            } else if frame >= 15.0 && frame < 17.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &olimar_hand_midpoint_scale);
            }
        }/* else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 {
            if frame > 9.0 && frame < 13.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handr"), &olimar_hand_scale);
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_scale);
            } else if frame >= 13.0 && frame < 15.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_midpoint_scale);
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_midpoint_scale);
            }
        } else if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4) {
            if frame > 10.0 && frame < 13.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_scale);
            } else if frame >= 13.0 && frame < 15.0 {
                ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("handl"), &olimar_hand_midpoint_scale);
            }
        }*/
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    winged_pikmin_cancel(boma, status_kind, cat[0]);
    pikmin_order_b_reverse(fighter);
    solimar_scaling(boma, status_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_PIKMIN )]
pub fn pikmin_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pikmin_frame(fighter)
    }
}

pub unsafe fn pikmin_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}