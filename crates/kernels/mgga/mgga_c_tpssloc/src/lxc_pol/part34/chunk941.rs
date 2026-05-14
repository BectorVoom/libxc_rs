//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 941/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk941<F: Float>(t214: F, t28199: F, t1985: F, t6460: F, t6906: F, t6889: F, t6347: F, t6890: F, t6888: F, t26193: F, t7691: F, t22933: F, t6439: F, t25: F, t5527: F, t1484: F, t1530: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28200 = t214 * t28199;
    let t28201 = t1985 * t28200;
    let t28205 = t6906 * t6460;
    let t28206 = t6889 * t28205;
    let t28207 = t1985 * t28206;
    let t28209 = t6890 * t6347;
    let t28210 = t6889 * t28209;
    let t28211 = t6888 * t28210;
    let t28213 = t26193 * t7691;
    let t28214 = t6888 * t28213;
    let t28232 = t22933 * t6439;
    let t28233 = t6889 * t28232;
    let t28234 = t1985 * t28233;
    let t28241 = t25 * t5527;
    let t28248 = t1484 * t1530;
    (t28200, t28201, t28205, t28206, t28207, t28209, t28210, t28211, t28213, t28214, t28232, t28233, t28234, t28241, t28248)
}
