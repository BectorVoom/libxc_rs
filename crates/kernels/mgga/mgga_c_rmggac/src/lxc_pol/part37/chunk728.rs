//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 728/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk728<F: Float>(t11670: F, t14236: F, t3369: F, t7834: F, t2144: F, t2816: F, t1971: F, t14258: F, t3148: F, t9221: F, t69808: F, t14125: F, t14131: F, t9158: F, t15379: F, t70337: F) -> (F, F, F, F, F) {
    let t75096 = t14236 * t3369 * t7834 * t11670;
    let t75098 = t2144 * t2816;
    let t75099 = t1971 * t75098;
    let t75100 = t14258 * t75099;
    let t75102 = t9221 * t3148;
    let t75103 = t75102 * t69808;
    let t75106 = t14131 * t14125 * t9158;
    let t75108 = t15379 * t70337;
    (t75096, t75100, t75103, t75106, t75108)
}
