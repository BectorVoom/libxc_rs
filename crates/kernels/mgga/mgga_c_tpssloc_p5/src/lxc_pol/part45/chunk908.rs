//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 908/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk908<F: Float>(t3701: F, t7216: F, t8639: F, t2233: F, t2239: F, t601: F, t9238: F, t85: F, t24: F, t12019: F, t566: F, t3700: F) -> (F, F, F, F, F, F, F) {
    let t32193 = t3701 * t7216;
    let t36740 = t3701 * t8639;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = F::cast_from(1.0_f64) / t12019 / t566;
    let t40610 = t3700 * t3700;
    (t32193, t36740, t39049, t39054, t39063, t40590, t40610)
}
