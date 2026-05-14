//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 779/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk779<F: Float>(t3806: F, t5248: F, t550: F, t31170: F, t553: F, t835: F, t544: F, t8467: F, t1369: F, t8466: F, t2314: F, t8326: F, t5113: F, t3938: F, t671: F, t3941: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31172 = t5248 * t3806 * t550;
    let t31173 = t31170 * t31172;
    let t31175 = t553 * t835;
    let t31176 = t544 * t31175;
    let t31177 = t31176 * t8467;
    let t31179 = t8466 * t1369;
    let t31236 = t2314 * t8326;
    let t31237 = 2.0 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0 * t31238;
    let t31283 = t3938 * t8326;
    let t31284 = 0.135e2 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    (t31172, t31173, t31175, t31176, t31177, t31179, t31237, t31239, t31284, t31285, t31286)
}
