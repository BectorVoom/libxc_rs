//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 937/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk937<F: Float>(t2902: F, t673: F, t2899: F, t2839: F, t57: F, t262: F, t390: F, t5543: F, t1016: F, t2193: F) -> (F, F, F, F, F, F) {
    let t9194 = t673 * t2902;
    let t9196 = t673 * t2899;
    let t9198 = t2839 * t57;
    let t9199 = F::new(1.0) / t9198;
    let t9213 = t262 * t5543 * t390;
    let t9214 = F::cast_from(0.93932222222222222223e0_f64) * t9213;
    let t9221 = t2193 * t1016;
    (t9194, t9196, t9199, t9213, t9214, t9221)
}
