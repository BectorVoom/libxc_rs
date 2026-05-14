//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 405/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk405<F: Float>(t1409: F, t3450: F, t135: F, t1716: F, t1174: F, t1714: F, t3448: F, t1721: F, t225: F, t1222: F, t1731: F, t1744: F, t1653: F, t248: F, t3521: F, t1227: F) -> (F, F, F, F, F, F, F, F) {
    let t4904 = t3450 * t1409;
    let t4916 = t135 * t1716;
    let t4917 = t1174 * t4916;
    let t4919 = t3448 * t1714;
    let t4945 = t1721 * t225;
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4993 = t248 * t3521 * t1653;
    let t4994 = t1227 * t4993;
    (t4904, t4917, t4919, t4945, t4957, t4959, t4993, t4994)
}
