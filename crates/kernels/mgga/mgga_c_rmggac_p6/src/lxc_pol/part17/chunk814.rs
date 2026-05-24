//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 814/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk814<F: Float>(t16501: F, t7363: F, t1966: F, t1180: F, t34759: F, t338: F, t615: F, t2186: F, t8582: F, t7288: F, t8659: F, t2286: F, t7921: F) -> (F, F, F, F, F, F, F) {
    let t39850 = t7363 * t16501;
    let t39851 = t1966 * t39850;
    let t39857 = t1180 * t34759;
    let t39866 = t338 * t615;
    let t39873 = t2186 * t8582;
    let t39874 = F::cast_from(0.19863479950205658386e-4_f64) * t39873;
    let t39899 = t8659 * t7288;
    let t39901 = t7921 * t2286;
    (t39850, t39851, t39857, t39866, t39874, t39899, t39901)
}
