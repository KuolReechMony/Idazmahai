//! Define shared behavior for the modules

use sqlx::Database;

use crate::errors::EntityError;

/// Any object that users can fully interact with (CRUD)
pub trait Entity<D: Database>: Sized {
    /// Object used to create valid entities
    type NewEntity: Sized;
    /// Object used to create and validate patches to entites
    type EntityPatch: Sized;

    fn insert(
        new: Self::NewEntity,
        db_transaction: sqlx::Transaction<D>,
    ) -> Result<Self, EntityError>;

    fn update(
        self,
        patch: Self::EntityPatch,
        db_transaction: sqlx::Transaction<D>,
    ) -> Result<Self, EntityError>;

    fn delete(self, db_transaction: sqlx::Transaction<D>) -> Result<(), EntityError>;
}
