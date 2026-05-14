//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 935/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk935<F: Float>(t23204: F, t6555: F, t23164: F, t6572: F, t6562: F, t2742: F, t6571: F, t6553: F, t1880: F, t2553: F, t6554: F, t6552: F, t212: F, t252: F, t23171: F, t23168: F, t6556: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23205 = t23204 * t6555;
    let t23206 = t23164 * t23205;
    let t23208 = t23204 * t6572;
    let t23209 = t6562 * t23208;
    let t23218 = t6571 * t2742;
    let t23219 = t6553 * t23218;
    let t23220 = t1880 * t23219;
    let t23222 = t6554 * t2553;
    let t23223 = t6553 * t23222;
    let t23224 = t6552 * t23223;
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23232 = t23168 * t6556;
    (t23205, t23206, t23208, t23209, t23218, t23219, t23220, t23222, t23223, t23224, t23228, t23229, t23230, t23232)
}
