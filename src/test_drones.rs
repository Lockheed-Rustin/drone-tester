#[macro_export]
macro_rules! test_drones {
    ($( $dep:ident :: $($p:ident)::+ )*) => {
        $(
            #[cfg(test)]
            mod $dep {
                #[test]
                fn test_fragment_forward() {
                    wg_2024::tests::generic_fragment_forward::<$dep$(::$p)*>();
                }

                #[test]
                fn test_fragment_drop() {
                    wg_2024::tests::generic_fragment_drop::<$dep$(::$p)*>();
                }
            }
        )*
    }
}
