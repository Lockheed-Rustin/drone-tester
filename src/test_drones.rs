#[macro_export]
macro_rules! test_drones {
    (@test $mod:ident ::{ $($f:ident),+ }) => {
        #[cfg(test)]
        mod $mod {
            use super::*;
            $(
                #[test]
                fn $f() {
                    $crate::with_timeout(
                        $crate::$mod::$f::<Drone>,
                        $crate::DEFAULT_TIMEOUT,
                    );
                }
            )*
        }
    };
    ($( $dep:ident :: $($p:ident)::+ )*) => {
        paste::paste!{
            $(
                #[cfg(test)]
                mod [<test_ $dep>] {
                    use super::test_drones;
                    type Drone = $dep$(::$p)*;

                    test_drones!(@test fragment::{double_chain, crash_double_chain});
                }
            )*
        }
    };
}
