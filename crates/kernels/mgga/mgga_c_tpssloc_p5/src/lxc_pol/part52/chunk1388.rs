//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1388/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1388<F: Float>(t25010: F, t8690: F, t116135: F, t25971: F, t120678: F, t120680: F, t120683: F, t120687: F, t120691: F, t120692: F, t120697: F, t120699: F, t120702: F, t1442: F, t27293: F, t31829: F, t6517: F) -> F {
    let t123228 = t8690 * t25010;
    let t123229 = t116135 * t25971;
    let t123232 = -t1442 * t31829 - F::new(2.0) * t27293 * t6517 - F::new(2.0) * t120678 - F::new(2.0) * t120680 - t120683 - t120687 - t120691 + F::new(3.0) * t120692 + t120697 + t120699 + t120702 - t123228 - F::new(3.0) * t123229;
    t123232
}
