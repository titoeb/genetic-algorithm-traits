// I allow unused imports here, because I just want to test in this file, that these
// two traits can be imported from the trait.
#[allow(unused_imports)]
use genetic_algorithm_traits::Individual;
#[allow(unused_imports)]
use genetic_algorithm_traits::Population;

#[test]
fn test_imports_have_run() {
    // If we get here, we have successfully imported
    // `Inidivual` as well as `Population` and are ready
    // to use it now.
    assert!(true);
}
