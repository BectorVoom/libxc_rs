//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2326/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2326<F: Float>(t157: F, t4196: F, t57973: F, t46439: F, t59004: F, t59013: F, t41291: F, t59022: F, t59024: F, t59028: F, t59032: F, t59037: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t67494 = F::new(36.0) * t57973 * t157 * t4196;
    let t67495 = F::new(3.0) * t46439;
    let t67496 = F::new(72.0) * t59004;
    let t67497 = F::new(36.0) * t59013;
    let t67498 = F::new(4.0) * t41291;
    let t67499 = F::new(72.0) * t59022;
    let t67500 = F::new(72.0) * t59024;
    let t67501 = F::cast_from(0.51947577317044391276e2_f64) * t59028;
    let t67502 = F::new(24.0) * t59032;
    let t67503 = F::new(12.0) * t59037;
    (t67494, t67495, t67496, t67497, t67498, t67499, t67500, t67501, t67502, t67503)
}
