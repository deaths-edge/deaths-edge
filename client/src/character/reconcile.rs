use bevy::prelude::*;

use common::{
    character::{CharacterIndex, CharacterMarker},
    network::server::Reconcile,
};

pub fn reconcile(
    mut reconcile_reader: EventReader<Reconcile>,
    mut character_query: Query<(&CharacterIndex, &mut Transform), With<CharacterMarker>>,
) {
    for reconcile in reconcile_reader.iter() {
        let result = character_query
            .iter_mut()
            .find(|(index, _)| **index == reconcile.index);
        if let Some((_, mut transform)) = result {
            transform.translation = reconcile.position.extend(0.);
        }
    }
}
