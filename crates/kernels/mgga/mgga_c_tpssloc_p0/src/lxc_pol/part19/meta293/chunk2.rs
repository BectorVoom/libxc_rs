//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1071/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1071<F: Float>(t1390: F, t3914: F, t3719: F, t571: F, t12048: F, t12051: F, t12053: F, t12055: F, t12057: F, t12059: F, t12085: F, t12087: F, t12090: F, t12092: F, t12094: F, t1307: F, t3918: F, t5126: F, t9789: F, t9793: F) -> (F, F) {
    let t12466 = t3914 * t1390;
    let t12470 = t571 * t3719;
    let t12474 = F::new(9.0) * t12466 * t1307 * t3918 + F::new(18.0) * t12470 * t1307 * t5126 - t12048 + t12051 + t12053 + t12055 - t12057 - t12059 + t12085 + t12087 - t12090 - t12092 - t12094 - t9789 + t9793;
    (t12466, t12474)
}
