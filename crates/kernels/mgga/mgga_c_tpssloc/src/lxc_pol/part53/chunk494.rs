//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 494/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk494<F: Float>(t3449: F, t4904: F, t3448: F, t461: F, t4729: F, t1178: F, t3966: F, t1177: F, t135: F, t1716: F, t1174: F, t1714: F, t3451: F, t3295: F, t3464: F, t4770: F, t4773: F, t4776: F, t4779: F) -> (F, F, F, F, F, F) {
    let t4905 = t3449 * t4904;
    let t4908 = t3448 * t461;
    let t4909 = t4908 * t4729;
    let t4912 = t1178 * t3966;
    let t4913 = t1177 * t4912;
    let t4916 = t135 * t1716;
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4920 = t4919 * t3451;
    let t4928 = -t3464 + t3295 / 9.0 + t4770 / 9.0 + t4773 / 18.0 - t4776 / 3.0 - t4779 / 6.0;
    (t4905, t4909, t4913, t4917, t4920, t4928)
}
