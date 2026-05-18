//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 840/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk840<F: Float>(t6920: F, t8462: F, t6600: F, t6932: F, t1338: F, t240: F, t241: F, t1336: F, t553: F, t835: F, t544: F, t8467: F) -> (F, F, F, F, F, F, F, F) {
    let t31153 = t6920 * t8462;
    let t31159 = t6600 * t8462;
    let t31160 = t6932 * t31159;
    let t31169 = t1338 * t240 * t241;
    let t31170 = t1336 * t31169;
    let t31175 = t553 * t835;
    let t31176 = t544 * t31175;
    let t31177 = t31176 * t8467;
    (t31153, t31159, t31160, t31169, t31170, t31175, t31176, t31177)
}
