//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1739/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1739<F: Float>(t85: F, t24: F, t12019: F, t566: F, t3700: F, t2751: F, t10108: F, t257: F, t1406: F, t9238: F, t2239: F, t3951: F) -> (F, F, F, F, F, F, F) {
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = F::new(1.0) / t12019 / t566;
    let t40610 = t3700 * t3700;
    let t40611 = F::new(1.0) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::new(1.0) / t40771;
    let t40889 = F::new(1.0) / t10108 / t257;
    let t45844 = t1406 * t9238;
    let t46104 = t3951 * t2239;
    (t39063, t40590, t40611, t40772, t40889, t45844, t46104)
}
