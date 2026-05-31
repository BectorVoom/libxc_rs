//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 514/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk514<F: Float>(t2105: F, t485: F, t190: F, t1992: F, t681: F, t680: F, t691: F) -> (F, F, F, F) {
    let t2106 = t485 * t2105;
    let t2109 = t190 * t1992;
    let t2111 = F::cast_from(4.0_f64) * t681 * t2109;
    let t2112 = t680 * t691;
    (t2106, t2109, t2111, t2112)
}
