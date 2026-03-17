//! Role types.

use twilight_model::guild::Role as TwilightRole;
use twilight_model::id::Id;
use twilight_model::id::marker::RoleMarker;

/// A Discord role.
pub struct Role {
    /// The role's ID.
    pub id: u64,
}

impl From<TwilightRole> for Role {
    fn from(value: TwilightRole) -> Self {
        Role { id: value.id.get() }
    }
}

/// A Discord role mention, together with the metadata sent by Discord with it.
pub struct RoleMention {
    /// The role's ID.
    pub id: u64,
}

impl From<Id<RoleMarker>> for RoleMention {
    fn from(value: Id<RoleMarker>) -> Self {
        Self { id: value.get() }
    }
}
