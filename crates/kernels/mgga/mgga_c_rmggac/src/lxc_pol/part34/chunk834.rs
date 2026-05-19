//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 834/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk834<F: Float>(t15098: F, t352: F, t1326: F, t70585: F, t69049: F, t15241: F, t4601: F, t15314: F, t56828: F, t69057: F, t3140: F, t3144: F, t9086: F) -> (F, F, F, F, F, F, F, F) {
    let t74977 = t15098 * t352;
    let t74978 = t1326 * t74977;
    let t74979 = t70585 * t74978;
    let t74981 = F::cast_from(0.15965655602485078085e0_f64) * t69049;
    let t74983 = F::cast_from(0.8980681276397856423e-1_f64) * t4601 * t15241;
    let t74984 = t56828 * t15314;
    let t74986 = F::cast_from(0.59590439850616975158e-4_f64) * t69057;
    let t74994 = t9086 * t3140 * t3144;
    (t74977, t74978, t74979, t74981, t74983, t74984, t74986, t74994)
}
