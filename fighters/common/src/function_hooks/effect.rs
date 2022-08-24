use super::*;
use globals::*;

const shockwave_fx: [u64 ; 2] = [hash40("sys_crown"), hash40("sys_crown_collision")];
const smoke_fx: [u64 ; 15] = [hash40("sys_atk_smoke"),
                            hash40("sys_atk_smoke2"),
                            hash40("sys_bound_smoke"),
                            hash40("sys_dash_smoke"),
                            hash40("sys_dash_smoke2"),
                            hash40("sys_down_smoke"),
                            hash40("sys_h_smoke_a"),
                            hash40("sys_h_smoke_b"),
                            hash40("sys_landing_smoke"),
                            hash40("sys_landing_smoke_s"),
                            hash40("sys_run_smoke"),
                            hash40("sys_sliding_smoke"),
                            hash40("sys_turn_smoke"),
                            hash40("sys_v_smoke_a"),
                            hash40("sys_v_smoke_b")];

//=================================================================
//== sv_animcmd::EFFECT
//== Note: Lua stack is 1-indexed, and "pop" means "get"
//=================================================================
#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT)]
unsafe fn EFFECT_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    let mut reduce_size = false;
    let mut reduce_alpha = false;

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 0 {
            let effect_name = hitbox_params[i as usize].get_hash();
            if shockwave_fx.contains(&effect_name.hash) {
                reduce_size = true;
            }
            if smoke_fx.contains(&effect_name.hash) {
                reduce_alpha = true;
            }
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
        // Index of effect size
        else if i == 8 && reduce_size {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    if reduce_alpha {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
        l2c_agent.clear_lua_stack();
    }
}

#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT_FOLLOW)]
unsafe fn EFFECT_FOLLOW_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 10] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..10 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    let mut reduce_size = false;
    let mut reduce_alpha = false;

    l2c_agent.clear_lua_stack();

    for i in 0..10 {
        // Index of effect name
        if i == 0 {
            let effect_name = hitbox_params[i as usize].get_hash();
            if shockwave_fx.contains(&effect_name.hash) {
                reduce_size = true;
            }
            if smoke_fx.contains(&effect_name.hash) {
                reduce_alpha = true;
            }
            //let mut aux: L2CValue = L2CValue::new_int(*ATTACK_LR_CHECK_POS as u64);
            //l2c_agent.push_lua_stack(&mut aux);
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        } 
        else if i == 8 && reduce_size {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    if reduce_alpha {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
        l2c_agent.clear_lua_stack();
    }
}

#[skyline::hook(replace=smash::app::sv_animcmd::EFFECT_FOLLOW_FLIP)]
unsafe fn EFFECT_FOLLOW_FLIP_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 12] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..12 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    let mut reduce_size = false;
    let mut reduce_alpha = false;

    l2c_agent.clear_lua_stack();

    for i in 0..12 {
        // Index of effect name
        if i == 0 {
            let effect_name = hitbox_params[i as usize].get_hash();
            if shockwave_fx.contains(&effect_name.hash) {
                reduce_size = true;
            }
            if smoke_fx.contains(&effect_name.hash) {
                reduce_alpha = true;
            }
            //let mut aux: L2CValue = L2CValue::new_int(*ATTACK_LR_CHECK_POS as u64);
            //l2c_agent.push_lua_stack(&mut aux);
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        } 
        else if i == 9 && reduce_size {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    if reduce_alpha {
        l2c_agent.clear_lua_stack();
        l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
        sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
        l2c_agent.clear_lua_stack();
    }
}

#[skyline::hook(replace=smash::app::sv_animcmd::FOOT_EFFECT)]
unsafe fn FOOT_EFFECT_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 8 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
}

#[skyline::hook(replace=smash::app::sv_animcmd::FOOT_EFFECT_FLIP)]
unsafe fn FOOT_EFFECT_FLIP_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 18] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..18 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }


    l2c_agent.clear_lua_stack();

    for i in 0..18 {
        // Index of effect name
        if i == 9 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
}

#[skyline::hook(replace=smash::app::sv_animcmd::LANDING_EFFECT)]
unsafe fn LANDING_EFFECT_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 16] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..16 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..16 {
        // Index of effect name
        if i == 8 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
}

#[skyline::hook(replace=smash::app::sv_animcmd::LANDING_EFFECT_FLIP)]
unsafe fn LANDING_EFFECT_FLIP_hook(lua_state: u64) {
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 18] = [L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void(), L2CValue::new_void()];

    for i in 0..18 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }


    l2c_agent.clear_lua_stack();

    for i in 0..18 {
        // Index of effect name
        if i == 9 {
            let size = hitbox_params[i as usize].get_f32();
            let mut new_size: L2CValue = L2CValue::new_num(size * 0.7);
            l2c_agent.push_lua_stack(&mut new_size);
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.7));
    sv_animcmd::LAST_EFFECT_SET_ALPHA(lua_state);
    l2c_agent.clear_lua_stack();
}

pub fn install() {
    skyline::install_hooks!(
        EFFECT_hook,
        EFFECT_FOLLOW_hook,
        EFFECT_FOLLOW_FLIP_hook,
        FOOT_EFFECT_hook,
        FOOT_EFFECT_FLIP_hook,
        LANDING_EFFECT_hook,
        LANDING_EFFECT_FLIP_hook
    );
}
