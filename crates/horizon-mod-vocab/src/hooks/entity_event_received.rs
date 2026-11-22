//! The `entity_event_received` hook: declarations the mod API generator reads (see `hooks/mod.rs`).

pub trait Hooks {
    /// The window received entity event `event` for `entity` from its server
    /// (`ClientboundEntityEventPacket`, `ClientPacketListener.handleEntityEvent`),
    /// heard before the client acts on it: 35 is a totem of undying saving the
    /// entity, 3 a death, 2 a hurt (vanilla's `EntityEvent` constants). Once for
    /// each packet, in single player too (its window is a client of the server
    /// thread), whether or not the client has `entity`: `host.entity_name(entity)`
    /// names a player the window can see. Client-side.
    #[hook(
        side = "client",
        contract = "listen",
        neoforge = "none (NeoForge has no event here; mods mixin into ClientPacketListener.handleEntityEvent)",
        fabric = "none (mods mixin into ClientPacketListener.handleEntityEvent)"
    )]
    fn entity_event_received();
}
