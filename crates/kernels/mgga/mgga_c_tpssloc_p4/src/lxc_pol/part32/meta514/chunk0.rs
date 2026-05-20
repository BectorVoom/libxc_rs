//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1844/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1844<F: Float>(t1799: F, t567: F, t1307: F, t22635: F, t26331: F, t1377: F, t1385: F, t22633: F, t22674: F, t7700: F, t6897: F, t1842: F, t6992: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26332 = t567 * t1799;
    let t26333 = t26332 * t1307;
    let t26334 = t22635 * t26333;
    let t26335 = t26331 * t26334;
    let t26337 = t1377 * t1799;
    let t26338 = t26337 * t1385;
    let t26339 = t22635 * t26338;
    let t26340 = t22633 * t26339;
    let t26344 = t22674 * t7700;
    let t26345 = t6897 * t26344;
    let t26347 = t6992 * t1842;
    (t26332, t26333, t26334, t26335, t26337, t26338, t26339, t26340, t26344, t26345, t26347)
}
