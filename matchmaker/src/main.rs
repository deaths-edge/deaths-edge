#![feature(int_abs_diff)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Rating(pub u32);

#[derive(Component)]
pub struct TwosTeam {
    player_a: Entity,
    player_b: Entity,
}

const MATCHING_DIFF: u32 = 100;

fn matchmaking_twos(
    team_query: Query<(Entity, &TwosTeam)>,
    rating_query: Query<&Rating>,
    mut commands: Commands,
) {
    for [(blue_id, blue_team), (red_id, red_team)] in team_query.iter_combinations::<2>() {
        let rating_blue = rating_query
            .get(blue_team.player_a)
            .expect("player not found")
            .0
            + rating_query
                .get(blue_team.player_b)
                .expect("player not found")
                .0;

        let rating_red = rating_query
            .get(red_team.player_a)
            .expect("player not found")
            .0
            + rating_query
                .get(red_team.player_b)
                .expect("player not found")
                .0;

        let rating_diff = rating_blue.abs_diff(rating_red);

        if rating_diff < MATCHING_DIFF {
            info!("matched");
            commands.entity(blue_id).despawn();
            commands.entity(red_id).despawn();
        }
    }
}

fn main() {
    App::new().add_plugins(MinimalPlugins).run()
}
