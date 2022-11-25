use std::sync::Arc;
use paste::paste;

use crate::{
    api::err::{ApiError, ApiErrorKind, ApiResult},
    auth::{AuthToken, JwtContext, AuthContext},
    config::Config,
    db::Transaction,
    search,
    prelude::*,
};


/// The context that is accessible to every resolver in our API.
pub(crate) struct Context {
    pub(crate) db: Transaction,
    pub(crate) auth: AuthContext,
    pub(crate) config: Arc<Config>,
    pub(crate) jwt: Arc<JwtContext>,
    pub(crate) search: Arc<search::Client>,
}

impl juniper::Context for Context {}

macro_rules! define_require_permission_wrapper {
    ($permission:ident, $key:expr) => {
        paste! {
            pub(crate) fn [<require_ $permission _permission>](&self) -> ApiResult<AuthToken> {
                self.auth.[<require_ $permission _permission>](&self.config.auth).ok_or_else(|| {
                    if let AuthContext::User(user) = &self.auth {
                        ApiError {
                            msg: format!(
                                concat!(
                                    "User '{}' does not have ",
                                    stringify!($permission),
                                    " permission",
                                ),
                                user.username
                            ),
                            kind: ApiErrorKind::NotAuthorized,
                            key: $key.map(|key| &*format!("{key}.not-authorized")),
                        }
                    } else {
                        ApiError {
                            msg: concat!(
                                stringify!($permission),
                                " permission required, but user is not logged in"
                            ).into(),
                            kind: ApiErrorKind::NotAuthorized,
                            key: $key.map(|key| &*format!("{key}.not-logged-in")),
                        }
                    }
                })
            }
        }
    }
}

impl Context {
    /// Returns a connection to the DB. Requires an auth token to prove the
    /// endpoint somehow handled authorization.
    pub(crate) fn db(&self, _: AuthToken) -> &Transaction {
        &self.db
    }

    define_require_permission_wrapper!(upload, Some("upload"));
    define_require_permission_wrapper!(moderator, Some("mutation"));
    define_require_permission_wrapper!(studio, None);
    define_require_permission_wrapper!(editor, None);
}
