//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1926/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1926<F: Float>(t28205: F, t6889: F, t1985: F, t6347: F, t6890: F, t6888: F, t26193: F, t7691: F, t1842: F, t7749: F, t3887: F, t2015: F, t6439: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28206 = t6889 * t28205;
    let t28207 = t1985 * t28206;
    let t28209 = t6890 * t6347;
    let t28210 = t6889 * t28209;
    let t28211 = t6888 * t28210;
    let t28213 = t26193 * t7691;
    let t28214 = t6888 * t28213;
    let t28219 = t7749 * t1842;
    let t28220 = t3887 * t28219;
    let t28223 = t2015 * t6439;
    (t28206, t28207, t28209, t28210, t28211, t28213, t28214, t28220, t28223)
}
