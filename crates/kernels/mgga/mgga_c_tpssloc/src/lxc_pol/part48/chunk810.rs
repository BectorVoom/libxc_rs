//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 810/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk810<F: Float>(t24802: F, t24866: F, t1241: F, t2144: F, t3481: F, t1190: F, t7348: F, t2154: F, t3630: F, t3598: F, t225: F, t7349: F) -> (F, F, F, F, F) {
    let t24867 = t24802 + t24866;
    let t24868 = t1241 * t24867;
    let t24871 = t3481 * t2144;
    let t24873 = t1190 * t7348;
    let t24876 = t2154 * t3630;
    let t24877 = t3598 * t24876;
    let t24880 = t7349 * t225;
    (t24868, t24871, t24873, t24877, t24880)
}
