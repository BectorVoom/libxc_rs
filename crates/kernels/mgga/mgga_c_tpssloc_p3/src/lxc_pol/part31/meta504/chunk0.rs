//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1700/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1700<F: Float>(t28213: F, t6888: F, t22933: F, t6439: F, t6889: F, t1985: F, t25: F, t5527: F, t1484: F, t1530: F) -> (F, F, F, F, F, F) {
    let t28214 = t6888 * t28213;
    let t28232 = t22933 * t6439;
    let t28233 = t6889 * t28232;
    let t28234 = t1985 * t28233;
    let t28241 = t25 * t5527;
    let t28248 = t1484 * t1530;
    (t28214, t28232, t28233, t28234, t28241, t28248)
}
