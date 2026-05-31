//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2354/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2354<F: Float>(t111: F, t8110: F, t12813: F, t16541: F, t2319: F, t24972: F, t5376: F, t7423: F, t85416: F, t86631: F, t86633: F, t86635: F, t86637: F, t86639: F, t86642: F, t86646: F, t86651: F, t86653: F, t86655: F, t86660: F, t86668: F, t91799: F, t91802: F) -> F {
    let t96334 = t8110 * t111;
    let t96337 = F::cast_from(27.0_f64) * t24972 * t16541 + t86631 + F::cast_from(0.135e2_f64) * t7423 * t12813 + t86633 + t86635 + t86637 + t86639 + t86642 + t86646 + t86651 + t86653 + t86655 + t86660 + t86668 + F::cast_from(54.0_f64) * t85416 * t5376 + F::cast_from(27.0_f64) * t96334 * t2319 + t91799 + t91802;
    t96337
}
