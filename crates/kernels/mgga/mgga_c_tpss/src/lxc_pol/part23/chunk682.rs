//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 682/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk682<F: Float>(t124: F, t3245: F, t762: F, t3234: F, t1218: F, t521: F) -> (F, F, F) {
    let t3246 = t124 * t3245;
    let t3247 = t762 * t3246;
    let t3251 = t762 * t124 * t3234;
    let t3255 = 1.0 / t1218 / t521;
    (t3247, t3251, t3255)
}
