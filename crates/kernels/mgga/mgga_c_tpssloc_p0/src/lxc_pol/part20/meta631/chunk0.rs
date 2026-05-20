//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2296/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2296<F: Float>(t1519: F, t2678: F, t10091: F, t13176: F, t13381: F, t13390: F, t13431: F, t13456: F, t255: F, t2617: F, t2738: F, t2740: F, t41014: F, t4162: F, t4166: F, t4281: F, t4282: F, t4291: F, t4295: F, t46861: F, t812: F, t9958: F, t9981: F) -> (F, F) {
    let t47528 = t1519 * t2678;
    let t47558 = F::new(2.0) * t41014 * t4281 * t4282 + F::new(14.0) * t4281 * t4282 * t9981 - t4282 * t4291 * t9958 - t4295 * t812 * t9958 - F::new(3.0) * t10091 * t4166 - F::new(3.0) * t13176 * t2738 - F::new(6.0) * t13381 * t13390 - F::new(6.0) * t13390 * t13456 - F::new(3.0) * t13431 * t2617 + t255 * t46861 + F::new(3.0) * t2740 * t4162;
    (t47528, t47558)
}
