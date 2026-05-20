//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2219/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2219<F: Float>(t12620: F, t12633: F, t12636: F, t12708: F, t1410: F, t1434: F, t2250: F, t2255: F, t2283: F, t2304: F, t3961: F, t3967: F, t3976: F, t4018: F, t608: F, t609: F, t642: F, t7445: F, t80: F, t9247: F, t9260: F, t9268: F, t9312: F) -> F {
    let t46080 = -t12633 * t642 / F::new(4.0) - t3967 * t2283 * t80 / F::new(4.0) - t1410 * t9312 * t80 / F::new(12.0) - t3976 * t2304 / F::new(4.0) - t9247 * t7445 * t2250 / F::new(4.0) - t9260 * t1434 / F::new(12.0) - t9268 * t1434 / F::new(4.0) - t2255 * t4018 / F::new(2.0) - t609 * t12620 / F::new(4.0) - t3961 * t2283 * t80 / F::new(4.0) - t608 * t12708 * t80 / F::new(4.0) - t12636 * t642 / F::new(2.0);
    t46080
}
