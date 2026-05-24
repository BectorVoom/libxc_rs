//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 739/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk739<F: Float>(t1327: F, t356: F, t640: F, t7323: F, t507: F, t8619: F, t22: F, t235: F, t29837: F, t1249: F, t2144: F, t7900: F, t892: F) -> (F, F, F, F, F) {
    let t34931 = t7323 * t640 * t356 * t1327;
    let t34938 = t507 * t8619;
    let t34944 = t235 * t29837 * t22;
    let t34957 = t1249 * t2144;
    let t34960 = t892 * t7900;
    (t34931, t34938, t34944, t34957, t34960)
}
