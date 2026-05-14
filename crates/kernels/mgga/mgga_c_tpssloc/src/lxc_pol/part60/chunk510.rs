//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 510/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk510<F: Float>(t221: F, t60: F, t3034: F, t334: F, t371: F, t2018: F, t532: F, t1984: F, t6546: F) -> (F, F, F, F, F) {
    let t6686 = t221 * t60;
    let t6739 = 1.0 / t3034 / t334;
    let t6793 = t371 * t334;
    let t6794 = 1.0 / t6793;
    let t6878 = t532 * t2018;
    let t6883 = t6546 * t1984;
    (t6686, t6739, t6794, t6878, t6883)
}
