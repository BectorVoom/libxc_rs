//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 586/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk586<F: Float>(t2189: F, t7228: F, t3350: F, t201: F, t4443: F, t2185: F, t7472: F, t16155: F, t7229: F, t507: F, t8619: F, t22: F, t235: F, t29837: F, t16502: F, t118: F, t1985: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34846 = t2189 * t7228;
    let t34847 = t34846 * t3350;
    let t34855 = t201 * t4443;
    let t34881 = t7472 * t2185;
    let t34884 = t7229 * t16155;
    let t34938 = t507 * t8619;
    let t34944 = t235 * t29837 * t22;
    let t34975 = t7229 * t16502;
    let t34976 = t1985 * t118;
    (t34846, t34847, t34855, t34881, t34884, t34938, t34944, t34975, t34976)
}
