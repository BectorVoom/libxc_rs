//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1044/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1044<F: Float>(t794: F, t8537: F, t6562: F, t23237: F, t8547: F, t1880: F, t2053: F, t2717: F) -> (F, F, F, F, F) {
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31321 = 0.41123351671205660912e-2 * t31320;
    let t31329 = t23237 * t8547;
    let t31330 = t1880 * t31329;
    let t31332 = t2717 * t2053;
    (t31319, t31321, t31329, t31330, t31332)
}
