//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 740/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk740<F: Float>(t18356: F, t3506: F, t11697: F, t6191: F, t3577: F, t248: F, t3570: F, t6219: F, t1213: F, t3521: F, t5975: F, t1227: F, t3450: F, t5398: F, t3448: F, t6138: F) -> (F, F, F, F, F, F) {
    let t18357 = t3506 * t18356;
    let t18371 = t11697 * t6191;
    let t18372 = t3577 * t18371;
    let t18375 = t248 * t3570 * t6219;
    let t18376 = t1213 * t18375;
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18409 = t3450 * t5398;
    let t18416 = t3448 * t6138;
    (t18357, t18372, t18376, t18393, t18409, t18416)
}
