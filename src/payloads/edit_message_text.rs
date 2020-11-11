// This file is auto generated by `cg` <https://github.com/teloxide/cg> (bc65e6f).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InlineKeyboardMarkup, Message, ParseMode};

impl_payload! {
    /// Use this method to edit text and [games] messages. On success, the edited Message is returned.
    ///
    /// See also: [`EditMessageTextInline`](crate::payloads::EditMessageTextInline)
    ///
    /// [games]: https://core.telegram.org/bots/api#games
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageText (EditMessageTextSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: ChatId [into],
            /// Identifier of the message to edit
            pub message_id: i64,
            /// New text of the message, 1-4096 characters after entities parsing
            pub text: String [into],
        }
        optional {
            /// Mode for parsing entities in the message text. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// Disables link previews for links in this message
            pub disable_web_page_preview: bool,
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
