//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 399/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk399<F: Float>(t7707: F, t793: F, t128: F, t830: F, t305: F, t648: F, t7561: F, t2068: F, t7638: F, t2067: F, t3839: F, t2073: F, t7645: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7815 = t793 * t7707;
    let t7816 = F::cast_from(0.15965655602485078085e0_f64) * t7815;
    let t7817 = t128 * t830;
    let t7818 = t305 * t7817;
    let t7819 = F::cast_from(0.14635184302277988245e0_f64) * t7818;
    let t7820 = t648 * t7561;
    let t7821 = F::cast_from(0.33335697577410973224e-1_f64) * t7820;
    let t7826 = t2068 * t7638;
    let t7829 = t3839 * t2067;
    let t7832 = t2073 * t7645;
    (t7815, t7816, t7818, t7819, t7820, t7821, t7826, t7829, t7832)
}
