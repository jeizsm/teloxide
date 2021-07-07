use crate::{adaptors::DefaultParseMode, requests::Requester, types::ParseMode};

#[cfg(feature = "cache_me")]
use crate::adaptors::CacheMe;

#[cfg(feature = "auto_send")]
use crate::adaptors::AutoSend;

#[cfg(feature = "throttle")]
use crate::adaptors::throttle::{Limits, Throttle};

/// Extensions methods for [`Requester`].
pub trait RequesterExt: Requester {
    /// Add `get_me` caching ability, see [`CacheMe`] for more.
    ///
    /// [`CacheMe`]:
    #[cfg(feature = "cache_me")]
    #[cfg_attr(
        all(any(docsrs, dep_docsrs), feature = "nightly"),
        doc(cfg(feature = "cache_me"))
    )]
    fn cache_me(self) -> CacheMe<Self>
    where
        Self: Sized,
    {
        CacheMe::new(self)
    }

    /// Send requests automatically, see [`AutoSend`] for more.
    #[cfg(feature = "auto_send")]
    #[cfg_attr(
        all(any(docsrs, dep_docsrs), feature = "nightly"),
        doc(cfg(feature = "auto_send"))
    )]
    fn auto_send(self) -> AutoSend<Self>
    where
        Self: Sized,
    {
        AutoSend::new(self)
    }

    /// Add throttling ability, see [`Throttle`] for more.
    ///
    /// Note: this spawns the worker, just as [`Throttle::new_spawn`].
    #[cfg(feature = "throttle")]
    #[cfg_attr(
        all(any(docsrs, dep_docsrs), feature = "nightly"),
        doc(cfg(feature = "throttle"))
    )]
    fn throttle(self, limits: Limits) -> Throttle<Self>
    where
        Self: Sized,
    {
        Throttle::new_spawn(self, limits)
    }

    /// Specifies default [`ParseMode`], which will be used during all calls to:
    ///
    ///  - [`send_message`]
    ///  - [`send_photo`]
    ///  - [`send_video`]
    ///  - [`send_audio`]
    ///  - [`send_document`]
    ///  - [`send_animation`]
    ///  - [`send_voice`]
    ///  - [`send_poll`]
    ///  - [`edit_message_text`] (and [`edit_message_text_inline`])
    ///  - [`edit_message_caption`] (and [`edit_message_caption_inline`])
    ///
    /// [`send_message`]: crate::requests::Requester::send_message
    /// [`send_photo`]: crate::requests::Requester::send_photo
    /// [`send_video`]: crate::requests::Requester::send_video
    /// [`send_audio`]: crate::requests::Requester::send_audio
    /// [`send_document`]: crate::requests::Requester::send_document
    /// [`send_animation`]: crate::requests::Requester::send_animation
    /// [`send_voice`]: crate::requests::Requester::send_voice
    /// [`send_poll`]: crate::requests::Requester::send_poll
    /// [`edit_message_text`]: crate::requests::Requester::edit_message_text
    /// [`edit_message_text_inline`]:
    /// crate::requests::Requester::edit_message_text_inline
    /// [`edit_message_caption`]:
    /// crate::requests::Requester::edit_message_caption
    /// [`edit_message_caption_inline`]:
    /// crate::requests::Requester::edit_message_caption_inline
    fn parse_mode(self, parse_mode: ParseMode) -> DefaultParseMode<Self>
    where
        Self: Sized,
    {
        DefaultParseMode::new(self, parse_mode)
    }
}

impl<T> RequesterExt for T
where
    T: Requester,
{
    /* use default impls */
}
