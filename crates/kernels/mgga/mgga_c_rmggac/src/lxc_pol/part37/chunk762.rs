//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 762/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk762<F: Float>(t15363: F, t69568: F, t14236: F, t14237: F, t1528: F, t2067: F, t26: F, t15388: F, t68538: F, t3154: F, t38638: F, t15266: F, t16156: F) -> (F, F, F, F, F) {
    let t73807 = t69568 * t15363;
    let t73812 = t14236 * t14237 * t2067 * t26 * t1528;
    let t73814 = t68538 * t15388;
    let t73816 = t38638 * t3154;
    let t73817 = F::cast_from(0.19863479950205658386e-4_f64) * t73816;
    let t73819 = t16156 * t15266;
    (t73807, t73812, t73814, t73817, t73819)
}
