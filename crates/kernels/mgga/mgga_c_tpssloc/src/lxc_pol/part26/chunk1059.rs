//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1059/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1059<F: Float>(t1089: F, t1240: F, t1251: F, t607: F, t24601: F, t225: F, t3590: F, t497: F, t462: F, t3597: F, t3599: F, t7300: F, t2123: F, t3471: F, t11613: F, t1238: F, t2121: F, t2155: F, t24564: F, t24568: F, t24571: F, t24575: F, t24577: F, t24582: F, t24587: F, t24589: F, t24591: F, t24597: F, t3487: F, t3593: F, t3600: F, t7283: F, t7351: F, t7356: F, t7392: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24602 = t1240 * t1089;
    let t24603 = t607 * t1251;
    let t24604 = t24602 * t24603;
    let t24605 = t24601 * t24604;
    let t24611 = t3590 * t225 * t497;
    let t24612 = t462 * t24611;
    let t24615 = t225 * t3597;
    let t24616 = t24615 * t3599;
    let t24617 = t7300 * t24616;
    let t24626 = t3471 * t2123;
    let t24629 = -0.82246703342411321825e-2 * t7283 * t24564 - 0.16449340668482264365e-1 * t7283 * t24568 - 0.82246703342411321825e-2 * t7283 * t24571 - 0.54831135561607547884e-2 * t24575 - 0.54831135561607547884e-2 * t24577 + 4.0 * t3487 * t7356 + 4.0 * t1238 * t24582 - t24587 + 0.54831135561607547884e-2 * t24589 * t24591 + 0.36554090374405031923e-2 * t7283 * t24597 + 0.54831135561607547884e-2 * t24589 * t24605 + 4.0 * t3593 * t7356 + 0.82246703342411321825e-2 * t2121 * t24612 + 0.16449340668482264365e-1 * t7283 * t24617 - 2.0 * t3487 * t7392 + 2.0 * t7351 * t3600 - 2.0 * t11613 * t2155 - 0.82246703342411321825e-2 * t7283 * t24626;
    (t24602, t24604, t24605, t24611, t24615, t24616, t24617, t24626, t24629)
}
