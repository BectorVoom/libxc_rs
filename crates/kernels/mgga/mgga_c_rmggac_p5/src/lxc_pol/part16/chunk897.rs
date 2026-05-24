//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 897/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk897<F: Float>(t14243: F, t16503: F, t559: F, t8425: F, t14249: F, t8430: F, t7414: F, t9783: F, t10040: F, t2186: F, t2010: F, t38816: F, t8465: F) -> (F, F, F, F, F) {
    let t44878 = t16503 * t14243 * t559 * t8425;
    let t44882 = t16503 * t14249 * t559 * t8430;
    let t44886 = t7414 * t9783;
    let t44888 = t2186 * t10040;
    let t44891 = t2010 * t8465 * t38816;
    (t44878, t44882, t44886, t44888, t44891)
}
