//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1044/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1044<F: Float>(t193: F, t8421: F, t25374: F, t86716: F, t200: F, t8365: F, t25: F, t25353: F, t606: F, t7540: F, t2752: F, t32885: F, t1877: F, t2219: F, t8370: F, t25365: F, t25373: F) -> (F, F, F, F, F, F, F, F) {
    let t118376 = t193 * t8421;
    let t118377 = t86716 * t25374;
    let t118381 = t193 * t200 * t8365;
    let t118387 = t25 * t25353;
    let t118393 = t606 * t7540;
    let t118399 = t32885 * t2752;
    let t118406 = t1877 * t8370 * t2219;
    let t118407 = t25373 * t25365;
    (t118376, t118377, t118381, t118387, t118393, t118399, t118406, t118407)
}
