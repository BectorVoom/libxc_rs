//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 179/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk179<F: Float>(t385: F, t389: F, t409: F, t179: F, t978: F, t431: F, t171: F, t388: F, t433: F, t151: F, t5: F, t959: F, t7: F, t245: F, t395: F, t163: F, t394: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1031 = t385 * t389;
    let t1037 = t409 * t409;
    let t1038 = 1.0 / t1037;
    let t1040 = t179 * t179;
    let t1041 = 1.0 / t1040;
    let t1042 = t1038 * t978 * t1041;
    let t1044 = 0.17315859105681463759e2 * t431 * t1042;
    let t1045 = t388 * t171;
    let t1046 = t1045 * t433;
    let t1050 = 0.14764627977777777777e-2 * t5 * t959 * t151;
    let t1054 = t5 * t7;
    let t1055 = t245 * t395;
    let t1059 = t394 * t163;
    (t1031, t1038, t1041, t1044, t1046, t1050, t1054, t1055, t1059)
}
