//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 186/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk186<F: Float>(t388: F, t421: F, t155: F, t385: F, t389: F, t409: F, t179: F, t978: F, t431: F, t171: F, t433: F, t151: F, t5: F, t959: F) -> (F, F, F, F, F, F, F) {
    let t1028 = t388 * t421;
    let t1029 = t155 * t1028;
    let t1031 = t385 * t389;
    let t1037 = t409 * t409;
    let t1038 = F::cast_from(1.0_f64) / t1037;
    let t1040 = t179 * t179;
    let t1041 = F::cast_from(1.0_f64) / t1040;
    let t1042 = t1038 * t978 * t1041;
    let t1044 = F::cast_from(0.17315859105681463759e2_f64) * t431 * t1042;
    let t1045 = t388 * t171;
    let t1046 = t1045 * t433;
    let t1050 = F::cast_from(0.14764627977777777777e-2_f64) * t5 * t959 * t151;
    (t1029, t1031, t1038, t1041, t1044, t1046, t1050)
}
