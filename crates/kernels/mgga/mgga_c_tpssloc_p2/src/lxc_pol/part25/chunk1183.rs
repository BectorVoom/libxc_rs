//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1183/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1183<F: Float>(t23998: F, t6486: F, t1860: F, t23992: F, t6509: F, t22527: F, t22531: F, t22534: F, t23975: F, t6492: F, t7035: F, t83832: F, t84203: F, t84205: F, t84207: F, t84209: F, t84216: F, t84220: F, t84222: F) -> F {
    let t84224 = t6486 * t23998;
    let t84229 = t1860 * t23992 * t6509;
    let t84231 = F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t84203 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t84205 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t84207 - F::cast_from(5.0_f64) * t84209 * t6492 - F::cast_from(10.0_f64) * t23975 * t22527 - F::cast_from(5.0_f64) * t23975 * t22531 - F::cast_from(70.0_f64) * t84216 * t83832 - F::cast_from(80.0_f64) * t84220 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t84222 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t84224 - F::cast_from(2.0_f64) * t22534 * t7035 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t84229;
    t84231
}
