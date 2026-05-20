//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 931/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk931<F: Float>(t10143: F, t8565: F, t531: F, t8639: F, t22716: F, t8622: F, t2085: F, t212: F, t22642: F, t6890: F, t794: F, t22690: F, t31618: F) -> (F, F, F, F, F, F) {
    let t115027 = t8565 * t10143;
    let t115262 = t531 * t8639;
    let t115305 = t22716 * t8622;
    let t115306 = F::cast_from(0.63969658155208805863e-1_f64) * t115305;
    let t115330 = t22642 * t212 * t2085 * t6890;
    let t115331 = F::cast_from(0.82246703342411321824e-2_f64) * t115330;
    let t115352 = t794 * t2085;
    let t115390 = t22642 * t22690 * t31618;
    (t115027, t115262, t115306, t115331, t115352, t115390)
}
