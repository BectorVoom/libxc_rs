//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 777/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk777<F: Float>(t2019: F, t2169: F, t7926: F, t7334: F, t7932: F, t7936: F, t2190: F, t678: F, t7920: F, t2160: F, t49: F, t7933: F, t7935: F) -> (F, F, F, F, F, F) {
    let t36809 = t2019 * t7926 * t2169;
    let t36912 = t7334 * t7932;
    let t36913 = t36912 * t7936;
    let t36916 = t2190 * t7920 * t678;
    let t36920 = t2160 * t49;
    let t36922 = t7933 * t36920 * t7935;
    (t36809, t36912, t36913, t36916, t36920, t36922)
}
