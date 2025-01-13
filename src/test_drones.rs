#[macro_export]
macro_rules! test_drones {
    (@test $mod:ident ::{ $($f:ident),+ }) => {
        #[cfg(test)]
        mod $mod {
            use super::*;
            $(
                #[test]
                fn $f() {
                    $crate::$mod::$f::<Drone>($crate::DEFAULT_TIMEOUT);
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

                    test_drones!(@test fragment::{
                        receive,
                        avoid_crash,
                        crash,
                        error_in_routing,
                        error_destination_is_drone,
                        pdr
                    });

                    test_drones!(@test flood::{
                        double_chain,
                        star,
                        butterfly,
                        tree,
                        subnet_star,
                        subnet_triangle
                    });
                }
            )*
        }
    };
}
