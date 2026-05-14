//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1174/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1174<F: Float>(t6552: F, t6553: F, t6554: F, t9516: F, t23164: F, t23204: F, t23222: F, t23168: F, t23238: F, t22986: F, t23270: F, t2553: F, t857: F, t865: F, t23196: F, t6562: F) -> (F, F, F, F, F) {
    let t82169 = t6552 * t6553 * t6554 * t9516;
    let t82172 = t23164 * t23204 * t23222;
    let t82174 = t23168 * t23238;
    let t82179 = t22986 * t23270 * t857 * t2553 * t865;
    let t82182 = t6562 * t23204 * t23196;
    (t82169, t82172, t82174, t82179, t82182)
}
