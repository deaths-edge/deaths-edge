use crate::abilities::*;

pub struct Fireball {
    marker: AbilityMarker,

    global_cooldown: GlobalCooldown,

    power_cost: PowerCost,

    requires_target: RequiresTarget,
    cast_duration: CastDuration,
}

impl Fireball {
    pub fn new() -> Self {
        Self {
            marker: AbilityMarker,

            global_cooldown: GlobalCooldown,

            power_cost: PowerCost(20.),

            requires_target: RequiresTarget::Enemy,
            cast_duration: CastDuration(Duration::from_secs(1)),
        }
    }
}
