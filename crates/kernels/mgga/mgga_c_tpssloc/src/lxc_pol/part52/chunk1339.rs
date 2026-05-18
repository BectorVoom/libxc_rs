//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1339/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1339<F: Float>(t120386: F, t120424: F, t1985: F, t26202: F, t31137: F, t1799: F, t2006: F, t1307: F, t26331: F, t26446: F, t1992: F, t550: F, t6976: F, t90942: F) -> (F, F, F, F, F) {
    let t120425 = t120386 + t120424;
    let t120436 = F::new(0.16449340668482264365e-1) * t1985 * t31137 * t26202;
    let t120437 = t2006 * t1799;
    let t120441 = F::new(0.9869604401089358619e-1) * t26331 * t26446 * t120437 * t1307;
    let t120445 = F::new(0.16449340668482264365e-1) * t1992 * t6976 * t90942 * t550;
    (t120425, t120436, t120437, t120441, t120445)
}
