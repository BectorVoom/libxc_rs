//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1341/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1341<F: Float>(t10292: F, t20718: F, t1981: F, t22143: F, t38: F, t1290: F, t1859: F, t1861: F, t19342: F, t19352: F, t19388: F, t19396: F, t20728: F, t20769: F, t20772: F, t21146: F, t5489: F, t5976: F, t5979: F, t6073: F, t6080: F, t6472: F, t6475: F, t69281: F, t7690: F) -> (F,) {
    let t72920 = t10292 * t20718;
    let t72930 = t1981 * t38 * t22143;
    let t72949 = 5.0 / 3.0 * t72920 * t5489 + 2.0 / 3.0 * t19396 * t6472 + 5.0 / 3.0 * t20728 * t19388 + 2.0 / 3.0 * t19396 * t6475 + 5.0 / 6.0 * t72930 * t5489 + 2.0 / 3.0 * t6080 * t20772 + 20.0 * t7690 * t1290 * t1859 * t19342 - t69281 * t1861 / 6.0 - t21146 * t5976 / 6.0 - t21146 * t5979 / 6.0 - t19352 * t6472 / 3.0 - t6073 * t20769 / 3.0;
    (t72949,)
}
