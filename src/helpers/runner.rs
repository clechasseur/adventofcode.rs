macro_rules! build_runner_for_days {
    ( $($day:literal),+ ) => {
        ::paste::paste! {
            pub const RUNNERS: &[[fn() -> String; 2]] = &[
                $( [$crate::[<day_ $day>]::part_1, $crate::[<day_ $day>]::part_2], )+
            ];
        }
    };
}
