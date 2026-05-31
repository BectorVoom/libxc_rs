//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1247/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1247<F: Float>(t3701: F, t8639: F, t1390: F, t601: F, t9238: F, t85: F, t24: F, t12019: F, t566: F, t3700: F, t2751: F, t10108: F, t257: F) -> (F, F, F, F, F, F, F, F) {
    let t36740 = t3701 * t8639;
    let t37790 = t8639 * t1390;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = F::cast_from(1.0_f64) / t12019 / t566;
    let t40610 = t3700 * t3700;
    let t40611 = F::cast_from(1.0_f64) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::cast_from(1.0_f64) / t40771;
    let t40889 = F::cast_from(1.0_f64) / t10108 / t257;
    (t36740, t37790, t39054, t39063, t40590, t40611, t40772, t40889)
}
