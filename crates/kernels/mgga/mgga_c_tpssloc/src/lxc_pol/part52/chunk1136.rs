//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1136/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1136<F: Float>(t31949: F, t576: F, t111: F, t8692: F, t112: F, t31923: F, t1395: F, t8702: F, t580: F, t2022: F, t7426: F, t2029: F, t7415: F, t193: F, t8421: F, t25374: F, t86716: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116330 = t576 * t31949;
    let t116343 = t8692 * t111;
    let t116362 = t31923 * t112;
    let t116368 = t1395 * t8702;
    let t116375 = t31923 * t580;
    let t116377 = t2022 * t7426;
    let t116383 = t7415 * t2029;
    let t118376 = t193 * t8421;
    let t118377 = t86716 * t25374;
    (t116330, t116343, t116362, t116368, t116375, t116377, t116383, t118376, t118377)
}
