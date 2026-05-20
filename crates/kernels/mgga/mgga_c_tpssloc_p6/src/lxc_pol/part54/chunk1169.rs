//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1169/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1169<F: Float>(t31483: F, t31517: F, t113: F, t1874: F, t23938: F, t26977: F, t6525: F, t7042: F, t7217: F, t8643: F, t1983: F, t6876: F, t8644: F) -> (F, F, F, F, F, F, F, F) {
    let t31518 = t31483 + t31517;
    let t31519 = t113 * t31518;
    let t31521 = F::new(2.0) * t23938 * t1874;
    let t31523 = F::new(2.0) * t26977 * t1874;
    let t31525 = F::new(2.0) * t7042 * t6525;
    let t31526 = t7217 * t8643;
    let t31527 = t1983 * t31526;
    let t31531 = t6876 * t8644;
    (t31518, t31519, t31521, t31523, t31525, t31526, t31527, t31531)
}
