//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 908/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk908<F: Float>(t1451: F, t1979: F, t1982: F, t201: F, t589: F, t1856: F, t446: F, t36471: F, t656: F, t6583: F, t36634: F, t6586: F) -> (F, F, F, F) {
    let t45104 = t589 * t1451 * t201 * t1979 * t1982;
    let t45109 = t446 * t1856 * t201 * t1979 * t1982;
    let t45120 = t36471 * t656 * t6583;
    let t45123 = t36634 * t656 * t6586;
    (t45104, t45109, t45120, t45123)
}
