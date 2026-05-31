//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1122/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1122<F: Float>(t1441: F, t1873: F, t3701: F, t8488: F, t12461: F, t8492: F, t1390: F, t601: F, t9238: F, t85: F, t24: F, t12019: F, t566: F) -> (F, F, F, F, F, F, F, F) {
    let t33211 = t1441 * t1873;
    let t36363 = t3701 * t8488;
    let t36533 = t12461 * t8492;
    let t37589 = t8488 * t1390;
    let t37593 = t8492 * t3701;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = F::cast_from(1.0_f64) / t12019 / t566;
    (t33211, t36363, t36533, t37589, t37593, t39054, t39063, t40590)
}
