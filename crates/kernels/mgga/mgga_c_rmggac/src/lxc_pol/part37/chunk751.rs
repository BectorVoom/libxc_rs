//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 751/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk751<F: Float>(t1550: F, t2060: F, t40983: F, t69894: F, t27: F, t9151: F, t16064: F, t69609: F, t11703: F, t14236: F, t14249: F, t2067: F, t14117: F, t69839: F, t8456: F, t14116: F, t14125: F, t9170: F) -> (F, F, F, F, F, F) {
    let t75602 = 0.5987120850931904282e-1 * t1550 * t2060 * t40983;
    let t75607 = 0.79828278012425390427e-1 * t69894;
    let t75609 = t27 * t9151;
    let t75611 = t69609 * t16064 * t75609;
    let t75615 = t14236 * t14249 * t2067 * t11703;
    let t75620 = t69839 * t14117 * t8456;
    let t75623 = t14116 * t14125 * t9170;
    (t75602, t75607, t75611, t75615, t75620, t75623)
}
