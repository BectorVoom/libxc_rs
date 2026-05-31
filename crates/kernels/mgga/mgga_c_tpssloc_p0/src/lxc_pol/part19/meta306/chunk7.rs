//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1100/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1100<F: Float>(t2235: F, t2240: F, t2241: F, t2307: F, t39030: F, t39032: F, t39034: F, t39036: F, t39038: F, t39040: F, t39043: F, t39046: F, t39049: F, t39054: F, t39063: F, t39064: F, t39070: F, t39130: F, t39217: F, t605: F, t645: F, t86: F, t9228: F, t9231: F, t9239: F, t9240: F, t9243: F, t9342: F) -> F {
    let t39221 = (t39030 - t39032 + t39034 - t39036 + t39038 - t39040 + t39043) * t86 - F::cast_from(16.0_f64) * t39046 * t645 + F::cast_from(120.0_f64) * t39049 * t2241 - F::cast_from(24.0_f64) * t9228 * t2307 - F::cast_from(480.0_f64) * t39054 * t9240 + F::cast_from(240.0_f64) * t9231 * t9243 - F::cast_from(16.0_f64) * t2235 * t9342 + F::cast_from(840.0_f64) * t39063 * t39064 - F::cast_from(720.0_f64) * t9239 * t2241 * t2307 + F::cast_from(60.0_f64) * t2240 * t39070 + F::cast_from(80.0_f64) * t2240 * t645 * t9342 - F::cast_from(4.0_f64) * t605 * (t39130 + t39217);
    t39221
}
