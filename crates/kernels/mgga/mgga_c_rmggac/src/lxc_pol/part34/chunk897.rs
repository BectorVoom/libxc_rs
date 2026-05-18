//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 897/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk897<F: Float>(t15049: F, t2604: F, t15128: F, t352: F, t262: F, t8620: F, t1971: F, t3351: F, t7190: F, t8950: F, t7262: F, t8979: F) -> (F, F, F, F, F, F) {
    let t76075 = F::new(0.2993560425465952141e-1) * t2604 * t15049;
    let t76077 = t15128 * t352;
    let t76078 = t262 * t76077;
    let t76079 = t8620 * t76078;
    let t76084 = F::new(0.10215503974391481456e-3) * t3351 * t1971 * t7190 * t8950;
    let t76087 = t3351 * t1971 * t7262 * t8979;
    (t76075, t76077, t76078, t76079, t76084, t76087)
}
