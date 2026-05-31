//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1267/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1267<F: Float>(t3205: F, t5935: F, t36: F, t68: F, t1338: F, t1795: F, t1799: F, t6435: F, t1289: F, t1270: F, t3204: F, t10178: F, t536: F) -> (F, F, F, F, F, F, F, F) {
    let t24128 = t3205 * t5935;
    let t24289 = t68 * t36;
    let t25232 = t1795 * t1338;
    let t25315 = t1338 * t1799;
    let t25469 = t3205 * t6435;
    let t25752 = t24289 * t1289;
    let t26848 = t6435 * t1270;
    let t30366 = t3204 * t3204;
    let t30367 = F::cast_from(1.0_f64) / t30366;
    let t31297 = F::cast_from(1.0_f64) / t10178 / t536;
    (t24128, t25232, t25315, t25469, t25752, t26848, t30367, t31297)
}
