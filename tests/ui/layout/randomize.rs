//@ build-pass
//@ revisions: normal randomize-layout
//@ [randomize-layout]compile-flags: -Zrandomize-layout

#![crate_type = "lib"]

struct Foo<T>(u32, T, u8);

struct Wrapper<T>(T);

#[repr(transparent)]
struct TransparentWrapper(u16);

const _: () = {
    // behavior of the current implementation, not guaranteed
    #[cfg(not(randomize_layout))]
    assert!(std::mem::offset_of!(Foo::<u16>, 1) == std::mem::offset_of!(Foo::<Wrapper<u16>>, 1));

    // under randomization Foo<T> != Foo<U>
    #[cfg(randomize_layout)]
    assert!(std::mem::offset_of!(Foo::<u16>, 1) != std::mem::offset_of!(Foo::<Wrapper<u16>>, 1));

    // Currently there are open questions about which repr(Rust) layouts are guranteed. Substituting
    // one type with a repr(transparent) wrapper one resulting in the same layout is a candidate for
    // one of the few things that might be guaranteed (but it currently is not).
    // Until this question is settled we don't gratiously break user code under randomization.
    #[cfg(randomize_layout)]
    assert!(
        std::mem::offset_of!(Foo::<u16>, 1) == std::mem::offset_of!(Foo::<TransparentWrapper>, 1)
    );
};
