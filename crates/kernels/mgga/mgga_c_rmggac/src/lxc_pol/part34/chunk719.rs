//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 719/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk719<F: Float>(t68815: F, t74968: F, t15098: F, t333: F, t1326: F, t1322: F, t235: F, t26115: F, t352: F, t70585: F, t69049: F, t15241: F, t4601: F, t15314: F, t56828: F, t69057: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t74969 = t68815 * t74968;
    let t74973 = t15098 * t333;
    let t74974 = t1326 * t74973;
    let t74975 = t235 * t26115 * t1322 * t74974;
    let t74977 = t15098 * t352;
    let t74978 = t1326 * t74977;
    let t74979 = t70585 * t74978;
    let t74981 = 0.15965655602485078085e0 * t69049;
    let t74983 = 0.8980681276397856423e-1 * t4601 * t15241;
    let t74984 = t56828 * t15314;
    let t74986 = 0.59590439850616975158e-4 * t69057;
    (t74969, t74973, t74974, t74975, t74977, t74978, t74979, t74981, t74983, t74984, t74986)
}
