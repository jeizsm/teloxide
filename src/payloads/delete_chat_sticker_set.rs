// This file is auto generated by `cg` <https://github.com/teloxide/cg> (bc65e6f).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field `can_set_sticker_set` optionally returned in [`GetChat`] requests to check if the bot can use this method. Returns _True_ on success.
    ///
    /// [`GetChat`]: crate::payloads::GetChat
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub DeleteChatStickerSet (DeleteChatStickerSetSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
        }
    }
}
