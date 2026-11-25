//! Define shared behavior for the modules

use sqlx::{Database, SqliteConnection};

use crate::errors::EntityError;

/// Models interaction with the [`Entity`] (s) should follow the model defined below:
///
/// ## Models
///
/// The application categorizes objects into the following categories
/// 1. [`Entity`]: Any object that user can `crud`
/// 2. [`EnumEntity`]: Any object that serves as variants to an input field but needs the user's input
/// 3. [`FeedEntity`]: Any `read-only` object that the user `subscribes` to
///
/// ## Allowed operations
///
/// The following operations are permitted on an [`Entity`]
/// 1. Make new [`Entity`] using the correspoding [`NewEntity`](Entity::NewEntity)
/// 2. View/Read a subset or all of the [`Entity`]
/// 3. Update [`Entity`] using the correspoding [`EntityPatch`](Entity::EntityPatch)
/// 4. Delete [`Entity`]
///
/// The following operations are permitted on an [`EnumEntity`]
/// 1. Make new [`EnumEntity`]
/// 2. View/Read a subset or all of the [`EnumEntity`]
/// 3. Update [`EnumEntity`]
/// 4. Delete [`EnumEntity`]
///
/// The following operations are permitted on a [`FeedEntity`]
/// 1. View/Read a subset or all of the [`FeedEntity`]
///
/// ## Behavior
///
/// Operations on objects should follow the template below:
///
/// ```
/// use idazmahai_core::{errors::EntityError, traits::Entity};
/// use sqlx::{Sqlite, SqliteConnection, SqlitePool};
///
/// pub struct SampleNewEntity { a: String, b: String };
/// impl SampleNewEntity {
///     fn new() -> Self {
///         Self { a: String::from("A"), b: String::from("B") }
///     }
/// }
/// pub struct SampleEntity { a: String, b: String, c: String };
/// pub struct SampleEntityPatch { a: String };
///
///
/// async fn new_obj(pool: SqlitePool) -> Result<SampleEntity, EntityError> {
///     let new = SampleEntity::new();
///     let mut tx = match pool.begin().await?;
///     match SampleEntity::insert(new, &mut tx).await? {
///         Ok(obj) => {
///             tx.commit().await?;
///             Ok(obj)
///         },
///         Err(err) => {
///             tx.rollback().await?;
///             Err(err)
///         }
///     }
/// }
///
/// fn update_obj(
///     original: SampleEntity,
///     update: SampleEntityPatch
/// ) -> Result<SampleEntity, EntityError> {
///     todo!()
/// }
///
/// fn delete_obj(
///     original: SampleEntity,
///     pool: SqlitePool
/// ) -> Result<(), EntityError> {
///     todo!()
/// }
///
/// impl Entity<Sqlite> for SampleEntity {
///     type NewEntity = SampleNewEntity;
///     type EntityPatch = SampleEntityPatch;
///
///     fn insert<'a>(
///         new: Self::NewEntity,
///         conn: &'a mut SqliteConnection
///     ) -> Result<Self, EntityError> {
///         let query = sqlx::query!(
///             "INSERT INTO sample_entity_table (val_a, val_b) VALUES ($1, $2)
///             RETURNING (val_a, val_b, val_c);");
///     }
///
///     fn update<'a>(
///         self,
///         patch: Self::EntityPatch,
///         conn: &'a mut SqliteConnection,
///     ) -> Result<Self, EntityError> {
///         todo!()
///     }
///     
///     fn delete<'a>(
///         self,
///         conn: &'a mut SqliteConnection
///     ) -> Result<(), EntityError> {
///         todo!()
///     }
/// }
///
///
/// ```
///

/// # Trait definition
///
/// Any object that users can fully interact with (CRUD)
///
/// # Example
/// ```
/// pub struct NewCourse {};
/// pub struct CoursePatch {};
/// pub struct Course {};
/// ```
pub trait Entity<D: Database>: Sized {
    /// Object used to create valid entities
    type NewEntity: Sized;
    /// Object used to create and validate patches to entites
    type EntityPatch: Sized;

    /// Takes singular [`NewEntity`](Entity::NewEntity) and return [`Entity`] upon successful `insert into db`
    async fn insert<'a>(
        new: Self::NewEntity,
        conn: &'a mut SqliteConnection,
    ) -> Result<Self, EntityError>;

    /// Takes singular [`Entity`] and a patch
    async fn update<'a>(
        self,
        patch: Self::EntityPatch,
        conn: &'a mut SqliteConnection,
    ) -> Result<Self, EntityError>;

    async fn delete<'a>(self, conn: &'a mut SqliteConnection) -> Result<(), EntityError>;
}

/// Any object that users have as variants to some input
/// # Example
/// ```
/// pub struct Location {};
/// pub struct WeatherSeason {};
/// ```
pub trait EnumEntity<D: Database>: Sized {
    fn insert<'a>(self, conn: &'a mut SqliteConnection);

    fn select_all<'a>(conn: &'a mut SqliteConnection) -> Vec<Self>;
}

/// `Read-only` objects fetched from online sources
/// # Example
/// ```
/// pub struct WeatherForecast {};
/// ```
pub trait FeedEntity<D: Database>: Sized {
    fn select_all<'a>(conn: &'a mut SqliteConnection) -> Vec<Self>;
}
