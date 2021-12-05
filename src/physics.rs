use heron::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum WorldLayer {
    Spell,
    Environment,
    Character,
}
