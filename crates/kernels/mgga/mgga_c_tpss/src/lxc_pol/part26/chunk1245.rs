//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1245/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1245<F: Float>(t1289: F, t18322: F, t19213: F, t20760: F, t4573: F, t4579: F, t4596: F, t5971: F, t61: F, t72: F, t1679: F, t6090: F, t6471: F, t1860: F, t21165: F, t1675: F, t1861: F, t19232: F, t19349: F, t20713: F, t20719: F, t20728: F, t21116: F, t21123: F, t21129: F, t21133: F, t21136: F, t21139: F, t21146: F, t5966: F, t6073: F, t6077: F, t6080: F, t6472: F, t6475: F) -> (F, F, F, F, F, F) {
    let t22143 = 88.0 / 9.0 * t4596 * t61 + 40.0 / 9.0 * t20760 * t1289 + 5.0 / 18.0 * t19213 * t4573 - 5.0 / 6.0 * t5971 * t4579 - t18322;
    let t22144 = t22143 * t72;
    let t22145 = t22144 * t1679;
    let t22148 = t6471 * t6090;
    let t22151 = t1860 * t21165;
    let t22174 = -5.0 * t19232 * t21116 - t21146 * t1861 / 6.0 - t6073 * t6472 / 3.0 - t6073 * t6475 / 3.0 - t1675 * t22145 / 6.0 - t1675 * t22148 / 3.0 - t1675 * t22151 / 6.0 - 10.0 / 3.0 * t19349 * t20713 + 5.0 / 3.0 * t20728 * t6077 + 2.0 / 3.0 * t21123 * t1861 + 5.0 / 3.0 * t20719 * t6077 + 5.0 / 3.0 * t5966 * t21129 + 5.0 / 6.0 * t5966 * t21133 + t21136 * t1861 / 3.0 + t21139 * t1861 / 3.0 + 2.0 / 3.0 * t6080 * t6472 + 2.0 / 3.0 * t6080 * t6475;
    (t22143, t22144, t22145, t22148, t22151, t22174)
}
