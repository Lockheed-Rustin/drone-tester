#[macro_export]
macro_rules! test_drones {
    (@test $mod:ident ::{ $($f:ident),+ }) => {
        #[cfg(test)]
        mod $mod {
            use super::Drone;
            $(
                #[test]
                fn $f() {
                    rayon::scope(|s| {
                        $crate::$mod::$f::<Drone>(s, $crate::DEFAULT_TIMEOUT);
                    });
                }
            )*
        }
    };
    ($( $dep:ident :: $($p:ident)::+, )*) => {
        paste::paste!{
            $(
                #[cfg(test)]
                mod [<test_ $dep>] {
                    type Drone = $dep$(::$p)*;

                    $crate::test_drones!(@test fragment::{
                        forward,
                        avoid_crash,
                        crash,
                        error_in_routing,
                        destination_is_drone,
                        pdr,
                        unexpected_recipient,
                        dropped_packets_during_crash
                    });

                    $crate::test_drones!(@test flood::{
                        double_chain,
                        double_chain_no_initiator,
                        star,
                        star_no_initiator,
                        butterfly,
                        butterfly_no_initiator,
                        tree,
                        tree_no_initiator,
                        subnet_star,
                        subnet_star_no_initiator,
                        subnet_triangle,
                        subnet_triangle_no_initiator
                    });

                    $crate::test_drones!(@test controller::{
                        packet_sent,
                        packet_dropped,
                        shortcut,
                        no_neighbor_after_drop,
                        shortcut_packets_during_crash
                    });
                }
            )*
        }
    };
}
