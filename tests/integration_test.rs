// Integration test for SigmaOS-cups-filters
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use cups_filters as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
