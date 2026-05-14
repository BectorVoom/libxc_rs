//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1131/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1131<F: Float>(t19506: F, t935: F, t5570: F, t6259: F, t1232: F, t1656: F, t520: F, t1265: F, t1640: F, t1258: F, t1771: F, t12828: F, t12823: F, t196: F, t197: F, t4352: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19507 = t19506 * t935;
    let t19509 = t6259 * t5570;
    let t19521 = t1656 * t1232 * t520;
    let t19535 = t1640 * t1265;
    let t19539 = t5570 * t1258;
    let t19540 = t1771 * t19539;
    let t19542 = t12828 * t1232;
    let t19554 = t12823 * t520;
    let t19577 = t4352 * t196 * t197;
    (t19507, t19509, t19521, t19535, t19539, t19540, t19542, t19554, t19577)
}
