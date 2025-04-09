// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct ResetBotsIfNoPlayersOnlineArgs {}

impl From<ResetBotsIfNoPlayersOnlineArgs> for super::Reducer {
    fn from(args: ResetBotsIfNoPlayersOnlineArgs) -> Self {
        Self::ResetBotsIfNoPlayersOnline
    }
}

impl __sdk::InModule for ResetBotsIfNoPlayersOnlineArgs {
    type Module = super::RemoteModule;
}

pub struct ResetBotsIfNoPlayersOnlineCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `reset_bots_if_no_players_online`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait reset_bots_if_no_players_online {
    /// Request that the remote module invoke the reducer `reset_bots_if_no_players_online` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_reset_bots_if_no_players_online`] callbacks.
    fn reset_bots_if_no_players_online(&self) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `reset_bots_if_no_players_online`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`ResetBotsIfNoPlayersOnlineCallbackId`] can be passed to [`Self::remove_on_reset_bots_if_no_players_online`]
    /// to cancel the callback.
    fn on_reset_bots_if_no_players_online(
        &self,
        callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
    ) -> ResetBotsIfNoPlayersOnlineCallbackId;
    /// Cancel a callback previously registered by [`Self::on_reset_bots_if_no_players_online`],
    /// causing it not to run in the future.
    fn remove_on_reset_bots_if_no_players_online(
        &self,
        callback: ResetBotsIfNoPlayersOnlineCallbackId,
    );
}

impl reset_bots_if_no_players_online for super::RemoteReducers {
    fn reset_bots_if_no_players_online(&self) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "reset_bots_if_no_players_online",
            ResetBotsIfNoPlayersOnlineArgs {},
        )
    }
    fn on_reset_bots_if_no_players_online(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext) + Send + 'static,
    ) -> ResetBotsIfNoPlayersOnlineCallbackId {
        ResetBotsIfNoPlayersOnlineCallbackId(self.imp.on_reducer(
            "reset_bots_if_no_players_online",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::ResetBotsIfNoPlayersOnline {},
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx)
            }),
        ))
    }
    fn remove_on_reset_bots_if_no_players_online(
        &self,
        callback: ResetBotsIfNoPlayersOnlineCallbackId,
    ) {
        self.imp
            .remove_on_reducer("reset_bots_if_no_players_online", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `reset_bots_if_no_players_online`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_reset_bots_if_no_players_online {
    /// Set the call-reducer flags for the reducer `reset_bots_if_no_players_online` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn reset_bots_if_no_players_online(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_reset_bots_if_no_players_online for super::SetReducerFlags {
    fn reset_bots_if_no_players_online(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("reset_bots_if_no_players_online", flags);
    }
}
