//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1163/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1163<F: Float>(t124: F, t12810: F, t762: F, t10111: F, t4415: F, t4416: F, t10117: F, t4425: F, t4466: F, t3261: F, t3273: F, t4471: F) -> (F, F, F, F, F) {
    let t12995 = t124 * t12810;
    let t12996 = t762 * t12995;
    let t13000 = t4415 * t4416 * t10111;
    let t13004 = F::new(7.0) / F::new(576.0) * t10117 * t4425;
    let t13006 = F::new(7.0) / F::new(2304.0) * t10117 * t4466;
    let t13009 = t3273 * t4471 * t3261;
    (t12996, t13000, t13004, t13006, t13009)
}
