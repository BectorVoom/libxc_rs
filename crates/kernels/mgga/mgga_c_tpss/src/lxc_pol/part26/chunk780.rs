//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 780/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk780<F: Float>(t30: F, t33: F, t1165: F, t1338: F, t3493: F, t4631: F, t4637: F, t4674: F, t93: F, t4356: F, t4358: F, t1288: F, t3282: F, t4578: F, t490: F, t1497: F, t3289: F, t493: F, t5059: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5322 = 2.0 * t1165 * t4674 + 4.0 * t1338 * t3493 + 2.0 * t4637 * t93 + t4631;
    let t5326 = 8.0 * t4356;
    let t5327 = 8.0 * t4358;
    let t5328 = t1288 * t1288;
    let t5334 = piecewise3(t31, 0.0, 4.0 / 9.0 * t3282 * t5328 + 4.0 / 3.0 * t490 * t4578);
    let t5335 = t1497 * t1497;
    let t5341 = piecewise3(t34, 0.0, 4.0 / 9.0 * t3289 * t5335 + 4.0 / 3.0 * t493 * t5059);
    (t5322, t5326, t5327, t5328, t5334, t5335, t5341)
}
