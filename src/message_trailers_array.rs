//! Bindings to libgit2's raw `git_message_trailer_array` type

use std::ops::Deref;

use crate::message_trailer::MessageTrailer;
use crate::raw;
use crate::util::Binding;
use std::mem;
use std::slice;

/// An message trailer array structure used by libgit2
///
/// Some apis return arrays of message trailers which originate from libgit2. This
/// wrapper type behaves a little like `Vec<&MessageTrailer>` but does so without copying
/// the underlying MessageTrailer until necessary.
pub struct MessageTrailerArray {
    raw: raw::git_message_trailer_array,
}

impl Deref for MessageTrailerArray {
    type Target = [MessageTrailer];

    fn deref(&self) -> &[MessageTrailer] {
        unsafe {
            debug_assert_eq!(mem::size_of::<MessageTrailer>(), mem::size_of_val(&*self.raw.trailers));

            slice::from_raw_parts(self.raw.trailers as *const MessageTrailer, self.raw.count as usize)
        }
    }
}

impl Binding for MessageTrailerArray {
    type Raw = raw::git_message_trailer_array;
    unsafe fn from_raw(raw: raw::git_message_trailer_array) -> MessageTrailerArray {
        MessageTrailerArray { raw: raw }
    }
    fn raw(&self) -> raw::git_message_trailer_array {
        self.raw
    }
}

impl Drop for MessageTrailerArray {
    fn drop(&mut self) {
        unsafe { raw::git_message_trailer_array_free(&mut self.raw) }
    }
}