//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2679/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2679<F: Float>(t54405: F, t12466: F, t1297: F, t15868: F, t15904: F, t1799: F, t193: F, t19577: F, t19596: F, t19994: F, t20077: F, t3719: F, t3914: F, t3918: F, t3919: F, t39476: F, t5126: F, t5160: F, t55191: F, t55266: F, t56219: F, t56275: F, t6301: F, t6347: F) -> (F, F) {
    let t56279 = F::cast_from(8.0_f64) * t54405;
    let t56294 = F::cast_from(3.0_f64) * t12466 * t3918 * t6347 + F::cast_from(3.0_f64) * t1297 * t193 * t56275 - F::cast_from(12.0_f64) * t15868 * t19577 * t3918 - F::cast_from(6.0_f64) * t15904 * t19596 * t3918 + F::cast_from(6.0_f64) * t1799 * t3918 * t55191 - t19596 * t3914 * t5160 + F::cast_from(12.0_f64) * t19994 * t3919 * t5126 - F::cast_from(3.0_f64) * t20077 * t3719 * t3918 + F::cast_from(12.0_f64) * t55266 * t6301 - t39476 - t56219 - t56279;
    (t56279, t56294)
}
