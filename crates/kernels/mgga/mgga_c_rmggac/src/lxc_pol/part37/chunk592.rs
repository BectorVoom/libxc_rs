//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 592/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk592<F: Float>(t1540: F, t2144: F, t5058: F, t511: F, t109: F, t24890: F, t5011: F, t534: F, t7350: F, t4617: F, t507: F, t338: F, t6444: F, t26: F, t7834: F, t797: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39953 = t1540 * t2144;
    let t40138 = t5058 * t511;
    let t40167 = t24890 * t109;
    let t40193 = t5011 * t511;
    let t40717 = t7350 * t534;
    let t40724 = t507 * t4617;
    let t40826 = t6444 * t338;
    let t40927 = t7834 * t26;
    let t40928 = t797 * t40927;
    (t39953, t40138, t40167, t40193, t40717, t40724, t40826, t40927, t40928)
}
