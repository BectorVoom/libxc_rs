//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 941/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk941<F: Float>(t17859: F, t9184: F, t3351: F, t515: F, t6561: F, t9188: F, t2144: F, t3352: F, t6564: F, t6523: F, t875: F, t1931: F, t1986: F) -> (F, F, F, F, F) {
    let t45648 = t17859 * t9184;
    let t45656 = t3351 * t9188 * t515 * t6561;
    let t45660 = t3351 * t3352 * t2144 * t6564;
    let t45664 = t3351 * t3352 * t875 * t6523;
    let t45666 = t1986 * t1931;
    (t45648, t45656, t45660, t45664, t45666)
}
