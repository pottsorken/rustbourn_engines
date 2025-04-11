// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::bevy_transform_type::BevyTransform;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct UpdateBotPositionArgs {
    pub bevy_transform: BevyTransform,
    pub bot_id: u64,
    pub new_rotate_dir: f32,
}

impl From<UpdateBotPositionArgs> for super::Reducer {
    fn from(args: UpdateBotPositionArgs) -> Self {
        Self::UpdateBotPosition {
            bevy_transform: args.bevy_transform,
            bot_id: args.bot_id,
            new_rotate_dir: args.new_rotate_dir,
        }
    }
}

impl __sdk::InModule for UpdateBotPositionArgs {
    type Module = super::RemoteModule;
}

pub struct UpdateBotPositionCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_bot_position`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_bot_position {
    /// Request that the remote module invoke the reducer `update_bot_position` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_bot_position`] callbacks.
    fn update_bot_position(
        &self,
        bevy_transform: BevyTransform,
        bot_id: u64,
        new_rotate_dir: f32,
    ) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_bot_position`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdateBotPositionCallbackId`] can be passed to [`Self::remove_on_update_bot_position`]
    /// to cancel the callback.
    fn on_update_bot_position(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &BevyTransform, &u64, &f32) + Send + 'static,
    ) -> UpdateBotPositionCallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_bot_position`],
    /// causing it not to run in the future.
    fn remove_on_update_bot_position(&self, callback: UpdateBotPositionCallbackId);
}

impl update_bot_position for super::RemoteReducers {
    fn update_bot_position(
        &self,
        bevy_transform: BevyTransform,
        bot_id: u64,
        new_rotate_dir: f32,
    ) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "update_bot_position",
            UpdateBotPositionArgs {
                bevy_transform,
                bot_id,
                new_rotate_dir,
            },
        )
    }
    fn on_update_bot_position(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &BevyTransform, &u64, &f32)
            + Send
            + 'static,
    ) -> UpdateBotPositionCallbackId {
        UpdateBotPositionCallbackId(self.imp.on_reducer(
            "update_bot_position",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer:
                                super::Reducer::UpdateBotPosition {
                                    bevy_transform,
                                    bot_id,
                                    new_rotate_dir,
                                },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, bevy_transform, bot_id, new_rotate_dir)
            }),
        ))
    }
    fn remove_on_update_bot_position(&self, callback: UpdateBotPositionCallbackId) {
        self.imp
            .remove_on_reducer("update_bot_position", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `update_bot_position`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_update_bot_position {
    /// Set the call-reducer flags for the reducer `update_bot_position` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn update_bot_position(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_update_bot_position for super::SetReducerFlags {
    fn update_bot_position(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("update_bot_position", flags);
    }
}
