//! Some libxkbcommon bindings.
//!
//! We do not wrap the full funcionality of xkb, as wlc handles
//! most of the setup.

/*
 * Copyright 1985, 1987, 1990, 1998  The Open Group
 * Copyright 2008  Dan Nicholson
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Except as contained in this notice, the names of the authors or their
 * institutions shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization from the authors.
 */

/************************************************************
 * Copyright (c) 1993 by Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, and distribute this
 * software and its documentation for any purpose and without
 * fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting
 * documentation, and that the name of Silicon Graphics not be
 * used in advertising or publicity pertaining to distribution
 * of the software without specific prior written permission.
 * Silicon Graphics makes no representation about the suitability
 * of this software for any purpose. It is provided "as is"
 * without any express or implied warranty.
 *
 * SILICON GRAPHICS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
 * SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT SHALL SILICON
 * GRAPHICS BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL
 * DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE
 * OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION  WITH
 * THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 ********************************************************/

/*
 * Copyright © 2009-2012 Daniel Stone
 * Copyright © 2012 Intel Corporation
 * Copyright © 2012 Ran Benita
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Author: Daniel Stone <daniel@fooishbar.org>
 */

use std::ffi::{CStr};
// Keysym utils functions

// An xkb keycode.
// Keycodes are handled by wlc
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Keycode(u32);

/// An xkb keysym.
///
/// # From xkb
/// A number used to represent the symbols generated from a key on a keyboard.
///
/// A key, represented by a keycode, may generate different symbols according
/// to keyboard state.  For example, on a QWERTY keyboard, pressing the key
/// labled \<A\> generates the symbol 'a'.  If the Shift key is held, it
/// generates the symbol 'A'.  If a different layout is used, say Greek,
/// it generates the symbol 'α'.  And so on.
///
/// Each such symbol is represented by a keysym.  Note that keysyms are
/// somewhat more general, in that they can also represent some "function",
/// such as "Left" or "Right" for the arrow keys.  For more information,
/// see:
/// http://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html#keysym_encoding
///
/// Specifically named keysyms can be found in the
/// `xkbcommon/xkbcommon-keysyms.h` header file.  Their name does not include
/// the XKB_KEY_ prefix.
///
/// Besides those, any Unicode/ISO 10646 character in the range U0100 to
/// U10FFFF can be represented by a keysym value in the range 0x01000100 to
/// 0x0110FFFF.  The name of Unicode keysyms is "U<codepoint>", e.g. "UA1B2".
///
/// The name of other unnamed keysyms is the hexadecimal representation of
/// their value, e.g. "0xabcd1234". Keysym names are case-sensitive.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keysym(u32);

/// Represents flags used for `Keysym::from_name`
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameFlags {
    /// None, or "Case sensitive"
    None = 0,
    /// Case insensitive name search
    CaseInsensitive = 1
}

#[link(name = "libxkbcommon")]
extern "C" {
    fn xkb_keysym_get_name(keysym: u32, buffer: *mut char, size: libc::size_t) -> i32;

    fn xkb_keysym_from_name(name: *const char, flags: KeyboardFlags) -> u32;

    fn xkb_keysym_to_utf8(keysym: u32, buffer: &mut char, size: libc::size_t) -> i32;

    fn xkb_keysym_to_utf32(keysym: u32) -> u32;
}

impl Keysym {

    /// Whether this keysym is valid or is `XKB_KEY_NoSymbol`
    pub fn is_valid(&self) {
        return self.0 != 0 && self.0 != 0xffffffff;
    }

    /// Gets the Keysym for the given name.
    ///
    /// # Arguments
    /// name: The name of a keysym. See docs for `get_name`.
    /// This function will accept any name returned by that function.
    ///
    /// flags: A set of flags controlling how the search is done.
    /// If the KeyboardFlags::CaseInsensitive flag is used and two keysym names
    /// differ only by case, then the lower-case keysym is returned.  For
    /// instance, for KEY_a and KEY_A, this function would return KEY_a for the
    /// case-insensitive search.  If this functionality is needed, it is
    /// recommended to first call this function without this flag, and if that
    /// fails, only then to try with this flag, while possibly warning the user
    /// that they have misspelled the name, and might get wrong results.
    ///
    /// returns: The keysym. If the name is invalid, returns None.
    ///
    /// # Examples
    /// ```rust
    /// let key_A = KeySym::from_name("A", KeyboardFlags::None);
    /// let key_a = KeySym::from_name("a", KeyboardFlags::None);
    /// let key_
    /// assert!(key.is_valid() && key.get_name() == "A");
    /// ```
    pub fn from_name(name: &str, flags: KeyboardFlags) -> Option<Keysym> {
        unsafe {
            let c_name = Cstr::new(name).unwrap() as *const char;
            let sym_val: u32 = xkb_keysym_from_name(c_name, flags);
            match sym_val {
                0 => None,
                _ => Some(Keysym(sym_val))
            }
        }
    }

    /// Gets name name of the keysym.
    ///
    /// # Examples
    /// ```rust
    /// let key = KeySym::from_name("A");
    /// assert!(key.is_valid() && key.get_name() == "A");
    /// ```
    pub fn get_name(&self) -> Option<String> {
        // create buffer
        // Call get_name with buffer
        // if get_name == -1 None
        // Convert buffer to String
        // The xkb documentation specifically recommends 7 as a buffer length
        const BUFFER_LEN: usize = 64usize;
        let buffer_vec: Vec<char> = Vec::with_capacity(BUFFER_LEN);
        unsafe {
            let mut buffer: &mut char = buffer_vec.as_mut_slice();
            let length = xkb_keysym_get_name(self.0, buffer, BUFFER_LEN);
            match length {
                -1 => None,
                _ => {
                    // Truncate buffer to = result!
                    //
                    str::from_utf8_lossy(buffer) // ?
                }
            }
        }
    }

    /// Gets the Unicode/UTF8 representation of this keysym.
    pub fn to_utf8() -> Option<String> {
        // create buffer
        // call to_utf8 with buffer
        // Convert buffer to String
        const BUFFER_LEN: usize = 7usize;
        let buffer_vec: Vec<char> = Vec::with_capacity(BUFFER_LEN);
        unsafe {
            let mut buffer: &mut char = buffer_vec.as_mut_slice();
            let result = xkb_keysym_get_name(self.0, buffer, BUFFER_LEN);
            match result {
                -1: None,
                _ => str::from_utf8_lossy(buffer)
            }
        }
    }

    /// Gets the Unicode/UTF32 representation of this keysym.
    pub fn to_utf32() -> u32 {
        
    }

}
