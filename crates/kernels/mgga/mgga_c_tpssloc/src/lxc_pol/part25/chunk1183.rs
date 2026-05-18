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
    let t84231 = F::new(32.0) / F::new(3.0) * t84203 + F::new(16.0) / F::new(3.0) * t84205 + F::new(32.0) / F::new(3.0) * t84207 - F::new(5.0) * t84209 * t6492 - F::new(10.0) * t23975 * t22527 - F::new(5.0) * t23975 * t22531 - F::new(70.0) * t84216 * t83832 - F::new(80.0) * t84220 - F::new(8.0) / F::new(3.0) * t84222 - F::new(16.0) / F::new(3.0) * t84224 - F::new(2.0) * t22534 * t7035 + F::new(88.0) / F::new(9.0) * t84229;
    t84231
}
