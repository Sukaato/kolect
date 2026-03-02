mod collectible;
mod dataset;
mod group;

/// export all entities
/// This allows us to import all entities from a single module, which is more convenient and cleaner.
pub use self::{
    collectible::Collectible, collectible::CollectibleDto, dataset::DatasetDto, group::Group,
    group::GroupDto,
};
