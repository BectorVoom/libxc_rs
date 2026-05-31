//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1283/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1283<F: Float>(t12645: F, t12718: F, t12566: F, t12568: F, t12571: F, t12582: F, t12585: F, t12588: F, t1437: F, t2235: F, t2240: F, t2241: F, t2307: F, t3953: F, t3958: F, t4021: F, t605: F, t645: F, t86: F, t9228: F, t9231: F, t9239: F) -> (F, F) {
    let t12719 = t12645 + t12718;
    let t12722 = t12566 * t86 - F::cast_from(8.0_f64) * t12568 * t645 + F::cast_from(20.0_f64) * t12571 * t2241 - F::cast_from(120.0_f64) * t12582 * t9239 + F::cast_from(40.0_f64) * t12585 * t2240 + F::cast_from(20.0_f64) * t12588 * t2240 - F::cast_from(4.0_f64) * t12719 * t605 - F::cast_from(4.0_f64) * t1437 * t9228 - F::cast_from(8.0_f64) * t2235 * t4021 - F::cast_from(4.0_f64) * t2307 * t3953 + F::cast_from(40.0_f64) * t3958 * t9231;
    (t12719, t12722)
}
