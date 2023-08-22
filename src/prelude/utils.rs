use std::fmt::Debug;

use bson::{Bson, to_bson};
use serde::Serialize;

use super::{BotResult, error::BsonSerSnafu};

// fn get_type<T>(_: &T) -> BotResult<String> {
//     Ok(std::any::type_name::<T>().to_string())
// }

/// Extension trait for converting any [`Serialize`]-able type to BSON through a method.
pub trait SerializeExt: Serialize {
    /// Converts the value to [`Bson`].
    fn to_bson(&self) -> BotResult<Bson>;
}

impl<T> SerializeExt for T
where
    T: Serialize + Debug,
{
    fn to_bson(&self) -> BotResult<Bson> {
        snafu::ResultExt::with_context(to_bson(self), |_| BsonSerSnafu {
            debug: format!("{:?}", self),
        })
    }
}

pub fn sanitize_username(str: String) -> String {
    return str.replace("||", "\\|\\|");
}