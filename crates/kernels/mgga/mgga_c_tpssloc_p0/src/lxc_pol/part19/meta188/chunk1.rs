//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 845/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk845<F: Float>(t805: F, t9541: F, t2563: F, t2610: F, t119: F, t210: F, t9516: F, t10009: F, t10012: F, t10014: F, t10017: F, t10026: F, t10029: F, t10030: F, t10033: F, t249: F, t2643: F, t787: F, t9559: F) -> (F, F) {
    let t10036 = t9541 * t805;
    let t10038 = t2563 * t2610;
    let t10041 = t210 * t119 * t9516;
    let t10044 = t2643 * t10009 / F::new(256.0) - F::new(7.0) / F::new(1536.0) * t10012 + F::new(119.0) / F::new(4608.0) * t10014 + t10017 * t249 / F::new(3072.0) - t10026 - t10029 - F::new(7.0) / F::new(16.0) * t10030 - t9559 * t10033 / F::new(4.0) - F::new(35.0) / F::new(72.0) * t10036 + F::new(7.0) / F::new(48.0) * t10038 - t787 * t10041 / F::new(48.0);
    (t10041, t10044)
}
