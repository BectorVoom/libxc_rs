//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 779/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk779<F: Float>(t1332: F, t8465: F, t8467: F, t1338: F, t240: F, t241: F, t1336: F, t3806: F, t5248: F, t550: F, t553: F, t835: F, t544: F, t1369: F, t8466: F, t6883: F, t8480: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31165 = t1332 * t8465;
    let t31166 = t31165 * t8467;
    let t31169 = t1338 * t240 * t241;
    let t31170 = t1336 * t31169;
    let t31172 = t5248 * t3806 * t550;
    let t31173 = t31170 * t31172;
    let t31175 = t553 * t835;
    let t31176 = t544 * t31175;
    let t31177 = t31176 * t8467;
    let t31179 = t8466 * t1369;
    let t31192 = 0.38381794893125283518e-1 * t6883 * t8480;
    (t31165, t31166, t31169, t31170, t31172, t31173, t31175, t31176, t31177, t31179, t31192)
}
