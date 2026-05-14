//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1185/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1185<F: Float>(t19456: F, t8326: F, t26114: F, t26117: F, t12725: F, t1458: F, t6514: F, t1868: F, t4072: F, t24994: F, t8449: F, t22751: F, t32731: F, t22633: F, t22635: F, t31099: F, t5187: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120120 = t19456 * t8326;
    let t120121 = 2.0 * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = 2.0 * t120122;
    let t120124 = t26117 * t8326;
    let t120125 = 2.0 * t120124;
    let t120130 = t12725 * t8326;
    let t120131 = 2.0 * t120130;
    let t120145 = t6514 * t1458;
    let t120148 = t1868 * t4072;
    let t120172 = t8449 * t24994;
    let t120179 = t22751 * t32731;
    let t120180 = 0.76763589786250567037e-1 * t120179;
    let t120184 = 0.3289868133696452873e-1 * t22633 * t22635 * t31099 * t5187;
    (t120121, t120123, t120125, t120131, t120145, t120148, t120172, t120180, t120184)
}
