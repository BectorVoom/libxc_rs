//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 530/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk530<F: Float>(t1326: F, t15105: F, t13911: F, t15098: F, t13916: F, t3839: F, t3826: F, t13928: F, t556: F, t13931: F, t2842: F, t13937: F, t13940: F, t2367: F, t36: F, t2079: F, t262: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15106 = t1326 * t15105;
    let t15107 = t13911 * t15106;
    let t15109 = t1326 * t15098;
    let t15110 = t13916 * t15109;
    let t15112 = t3839 * t15105;
    let t15114 = t3826 * t15098;
    let t15116 = t13928 * t556;
    let t15118 = t13931 * t2842;
    let t15120 = t13937 * t15106;
    let t15122 = t13940 * t15109;
    let t15128 = t36 * t2367;
    let t15130 = t2079 * t262 * t15128;
    (t15106, t15107, t15109, t15110, t15112, t15114, t15116, t15118, t15120, t15122, t15128, t15130)
}
