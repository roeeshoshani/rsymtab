// #![no_std]

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.as_usize())
    }
}
impl core::fmt::Display for SymbolAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:x}", self.as_usize())
    }
}
