// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//  ucd-generate jamo-short-name ./ucd-13.0.0 --fst-dir benches/tables/fst
//
// ucd-generate 0.2.6 is available on crates.io.

lazy_static! {
    pub static ref JAMO_SHORT_NAME: ::fst::Map = ::fst::Map::from(
        ::fst::raw::Fst::from_static_slice(include_bytes!(
            "jamo_short_name.fst"
        ))
        .unwrap()
    );
}
