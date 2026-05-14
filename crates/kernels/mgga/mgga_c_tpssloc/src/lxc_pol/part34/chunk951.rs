//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 951/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk951<F: Float>(t5: F, t109: F, t28941: F, t112: F, t23912: F, t26127: F, t28012: F, t28014: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t110 = 1.0 < t109;
    let t28942 = piecewise3(t8, 0.0, t28941);
    let t28943 = t28942 * t112;
    let t28951 = piecewise3(t110, 0.0, t23912 + 4.0 / 3.0 * t26127 + t28012 / 2.0 - t28014 / 4.0);
    (t28942, t28943, t28951)
}
