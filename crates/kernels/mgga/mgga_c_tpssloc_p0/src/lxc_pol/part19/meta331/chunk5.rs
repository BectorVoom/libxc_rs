//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1186/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1186<F: Float>(t40041: F, t562: F, t12168: F, t12172: F, t12179: F, t12240: F, t12241: F, t12256: F, t12267: F, t12273: F, t1336: F, t1380: F, t16033: F, t16047: F, t16055: F, t22740: F, t3777: F, t3901: F, t3905: F, t40271: F, t40335: F, t40439: F, t5334: F, t564: F) -> (F, F) {
    let t40541 = t40041 * t562;
    let t40576 = -F::new(4.0) * t12168 * t1336 * t3901 + F::new(36.0) * t12240 * t22740 * t5334 - t1336 * t1380 * t40271 - F::new(36.0) * t16047 * t22740 * t40335 + F::new(24.0) * t12172 * t3777 - F::new(4.0) * t12179 * t3777 + F::new(24.0) * t12241 * t16055 + F::new(24.0) * t12256 * t3777 - F::new(6.0) * t12267 * t3905 - F::new(12.0) * t12273 * t16033 + t40439 * t564;
    (t40541, t40576)
}
