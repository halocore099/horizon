use super::*;

impl Game<'_> {
    /// The window received entity event `event` for `entity` from its server
    /// (`ClientPacketListener.handleEntityEvent`), heard before the client acts on it.
    /// Client-side.
    pub(in crate::game) fn native_entity_event_received(&mut self, entity: EntityRef, event: u8) {
        self.fire_entity_event_received(entity, event);
    }
}

#[cfg(test)]
mod entity_event_received_tests {
    use crate::game::native_entity_refs::tests::with_client_game;
    use horizon_protocol::{
        connection::Pipe,
        packets::play::{Clientbound as C, EntityEvent},
    };
    use mod_api::{ClientHost, EntityEventReceivedEvent, Hooks, ModEntry};
    use mod_platform::testing::with_mods;
    use std::sync::{Arc, Mutex};

    struct Heard(Arc<Mutex<Vec<(u32, u8)>>>);
    #[mod_api::__subscriptions]
    impl Hooks for Heard {
        fn entity_event_received(
            &self,
            _: &mut dyn ClientHost,
            event: &mut EntityEventReceivedEvent,
        ) {
            self.0
                .lock()
                .unwrap()
                .push((event.entity.id(), event.event));
        }
    }

    /// `ClientPacketListener.handleEntityEvent`: a client mod hears each entity event the
    /// server sends, the totem's 35 included, with the entity it names, known to the client or not.
    #[test]
    #[ignore = "daily"]
    fn a_client_mod_hears_each_entity_event_the_server_sends() {
        let heard = Arc::new(Mutex::new(Vec::new()));
        let mods: mod_api::Mods =
            vec![ModEntry::new("heard", Box::new(Heard(Arc::clone(&heard))))].into();
        with_mods(mods, || {
            with_client_game(|game| {
                let (client, _server) = Pipe::pair();
                game.connect_local(client, "EventTest").unwrap();
                game.spawn_entity(40, "minecraft:player", [5.0, 64.0, 7.0], [0.0; 3]);
                for (id, event) in [(40, 35), (40, 2), (99, 3)] {
                    game.handle_packet(C::EntityEvent(EntityEvent { id, event }))
                        .unwrap();
                }
            });
        });
        assert_eq!(*heard.lock().unwrap(), [(40, 35), (40, 2), (99, 3)]);
    }
}
