//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 812/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk812<F: Float>(t5953: F, t645: F, t117: F, t5815: F, t1279: F, t1281: F, t1851: F, t1853: F, t547: F, t548: F, t5947: F, t3418: F, t38: F) -> (F, F, F, F) {
    let t5954 = t5953 * t645;
    let t5957 = t117 * t5815;
    let t5960 = 3.0 * t1279 * t1853 + 3.0 * t1281 * t1851 + 6.0 * t547 * t5954 + 3.0 * t547 * t5957 + t548 * t5947;
    let t6073 = t3418 * t38;
    (t5954, t5957, t5960, t6073)
}
