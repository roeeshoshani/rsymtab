//! this is a crate for generating a list of exported symbols, similar to how the `ksymtab` works in the linux kernel, but for
//! rust crates.
//!
//! the basic idea is that you can export items by adding an `#[export]` attribute on them, and then you can access
//! all the exported symbols by calling the [`symbols`] function.
//!
//! # Example
//!
//! ```
//! fn main() {
//!     println!("{:?}", rsymtab::symbols());
//! }
//!
//! #[rsymtab::export]
//! fn foo() {}
//!
//! #[rsymtab::export]
//! fn bar() {}
//!
//! #[rsymtab::export]
//! static mut FOO: u32 = 5;
//! ```
//!
//! # Portability
//!
//! **NOTE: this crate currently only works on linux.**
//!
//! that is because it uses a linker script to achieve some of the magic of creating the symbol table.
//! additionally, only linkers which support specification of multiple linker scripts are supported, because otherwise this crate
//! will overwrite the default linker script.

#![no_std]

pub use rsymtab_macros::export;

extern "C" {
    static __start_rsymtab: *mut ();
    static __stop_rsymtab: *mut ();
}

fn align_down(value: usize, alignment: usize) -> usize {
    (value / alignment) * alignment
}

/// returns a list of all the exported symbols
pub fn symbols() -> &'static [RsymtabSymbol] {
    let start = unsafe { &__start_rsymtab } as *const _ as usize;
    let unaligned_end = unsafe { &__stop_rsymtab } as *const _ as usize;
    let unaligned_len_in_bytes = unaligned_end - start;
    let len_in_bytes = align_down(
        unaligned_len_in_bytes,
        core::mem::size_of::<RsymtabSymbol>(),
    );
    let len = len_in_bytes / core::mem::size_of::<RsymtabSymbol>();
    unsafe { core::slice::from_raw_parts(start as *const RsymtabSymbol, len) }
}

/// an empty placeholder static variable that is placed in the rsymtab section just to make sure that the section exists, to avoid
/// linker errors.
#[link_section = "rsymtab"]
#[allow(dead_code)]
static PLACE_HOLDER: () = ();

/// an exported symbol
#[derive(Debug)]
#[repr(C)]
pub struct RsymtabSymbol {
    /// the name of the symbol
    pub name: &'static str,

    /// the address of the symbol
    pub address: SymbolAddress,
}

/// the address of an exported symbol
#[repr(transparent)]
pub struct SymbolAddress(&'static ());
impl SymbolAddress {
    /// an internal function used for constructing a symbol address object from a reference.
    #[doc(hidden)]
    pub const fn _from_reference(reference: &'static ()) -> Self {
        Self(reference)
    }

    /// returns a pointer to the symbol
    pub fn as_ptr(&self) -> *const () {
        self.0 as *const ()
    }

    /// returns a mutable pointer to the symbol
    pub fn as_mut_ptr(&self) -> *mut () {
        self.0 as *const () as *mut ()
    }

    /// returns the address of the symbol as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.as_ptr() as usize
    }
}
impl core::fmt::Debug for SymbolAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:x}", self.as_usize())
    }
}
impl core::fmt::Display for SymbolAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:x}", self.as_usize())
    }
}
