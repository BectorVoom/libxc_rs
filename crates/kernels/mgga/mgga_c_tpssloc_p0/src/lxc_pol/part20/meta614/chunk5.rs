//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2209/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2209<F: Float>(t25: F, t28: F, t88: F, t9416: F, t1406: F, t9238: F, t16: F, t39031: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t45814 = t88 * t9416;
    let t45844 = t1406 * t9238;
    let t45869 = F::new(12.0) * t16;
    let t45870 = F::new(24.0) * t39031;
    let t45872 = piecewise5::<F>(t26, F::new(0.0), t29, F::new(0.0), -t45869 + t45870);
    (t45814, t45844, t45872)
}
