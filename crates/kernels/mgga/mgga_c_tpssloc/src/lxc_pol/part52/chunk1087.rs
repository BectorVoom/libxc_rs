//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1087/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1087<F: Float>(t225: F, t7723: F, t2015: F, t5353: F, t3887: F, t22897: F, t5336: F, t1992: F, t22751: F, t7733: F, t1799: F, t22881: F) -> (F, F, F, F, F) {
    let t26366 = t7723 * t225;
    let t26370 = t2015 * t5353;
    let t26371 = t3887 * t26370;
    let t26378 = t22897 * t5336;
    let t26379 = t1992 * t26378;
    let t26381 = t22751 * t7733;
    let t26384 = t22881 * t1799;
    (t26366, t26371, t26379, t26381, t26384)
}
