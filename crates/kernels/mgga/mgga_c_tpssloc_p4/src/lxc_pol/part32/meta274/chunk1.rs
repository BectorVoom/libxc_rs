//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1247/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1247<F: Float>(t1484: F, t1915: F, t202: F, t7540: F, t1530: F, t1877: F, t193: F, t2522: F, t6670: F, t870: F, t28: F, t1649: F, t7541: F) -> (F, F, F, F, F) {
    let t7634 = t1915 * t1484;
    let t7637 = t202 * t7540;
    let t7642 = -t1530 * t1877 * t6670 + t193 * t7637 * t870 + F::new(3.0) * t2522 * t7634;
    let t7649 = t28 * t1484;
    let t7650 = t1915 * t7649;
    let t7656 = t28 * t1530;
    let t7663 = F::new(3.0) / F::new(2.0) * t2522 * t7650 + t1877 * t7541 * t28 / F::new(2.0) - t1877 * t6670 * t7656 / F::new(2.0) + t1877 * t1915 * t1649 / F::new(2.0);
    (t7637, t7642, t7649, t7656, t7663)
}
