// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::block_type::Block;
use super::owner_type_type::OwnerType;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `block`.
///
/// Obtain a handle from the [`BlockTableAccess::block`] method on [`super::RemoteTables`],
/// like `ctx.db.block()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.block().on_insert(...)`.
pub struct BlockTableHandle<'ctx> {
    imp: __sdk::TableHandle<Block>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `block`.
///
/// Implemented for [`super::RemoteTables`].
pub trait BlockTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`BlockTableHandle`], which mediates access to the table `block`.
    fn block(&self) -> BlockTableHandle<'_>;
}

impl BlockTableAccess for super::RemoteTables {
    fn block(&self) -> BlockTableHandle<'_> {
        BlockTableHandle {
            imp: self.imp.get_table::<Block>("block"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct BlockInsertCallbackId(__sdk::CallbackId);
pub struct BlockDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for BlockTableHandle<'ctx> {
    type Row = Block;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = Block> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = BlockInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> BlockInsertCallbackId {
        BlockInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: BlockInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = BlockDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> BlockDeleteCallbackId {
        BlockDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: BlockDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<Block>("block");
    _table.add_unique_constraint::<u64>("id", |row| &row.id);
}
pub struct BlockUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for BlockTableHandle<'ctx> {
    type UpdateCallbackId = BlockUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> BlockUpdateCallbackId {
        BlockUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: BlockUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<Block>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<Block>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `id` unique index on the table `block`,
/// which allows point queries on the field of the same name
/// via the [`BlockIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.block().id().find(...)`.
pub struct BlockIdUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<Block, u64>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> BlockTableHandle<'ctx> {
    /// Get a handle on the `id` unique index on the table `block`.
    pub fn id(&self) -> BlockIdUnique<'ctx> {
        BlockIdUnique {
            imp: self.imp.get_unique_constraint::<u64>("id"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> BlockIdUnique<'ctx> {
    /// Find the subscribed row whose `id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u64) -> Option<Block> {
        self.imp.find(col_val)
    }
}
