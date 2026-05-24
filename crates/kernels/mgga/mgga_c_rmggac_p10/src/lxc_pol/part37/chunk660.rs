//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 660/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk660<F: Float>(t3350: F, t39207: F, t623: F, t7191: F, t1679: F, t7203: F, t16501: F, t7363: F, t1966: F, t1540: F, t2144: F, t5058: F, t511: F) -> (F, F, F, F, F, F) {
    let t39277 = t39207 * t3350;
    let t39570 = t623 * t7191;
    let t39705 = t1679 * t7203;
    let t39850 = t7363 * t16501;
    let t39851 = t1966 * t39850;
    let t39953 = t1540 * t2144;
    let t40138 = t5058 * t511;
    (t39277, t39570, t39705, t39851, t39953, t40138)
}
