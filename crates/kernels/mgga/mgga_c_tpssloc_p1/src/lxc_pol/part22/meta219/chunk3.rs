//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1265/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1265<F: Float>(t3: F, t6470: F, t1401: F, t1458: F, t3941: F, t5371: F, t5456: F, t5493: F, t577: F, t641: F, t71: F, t154: F, t781: F) -> (F, F, F, F) {
    let t6471 = t3 * t6470;
    let t6483 = F::new(0.45e1) * t6470 * t577 + F::new(27.0) * t5371 * t1458 + F::new(27.0) * t3941 * t5456 + F::new(0.135e2) * t1401 * t5493;
    let t6509 = t71 * t641;
    let t6546 = t781 * t154;
    (t6471, t6483, t6509, t6546)
}
