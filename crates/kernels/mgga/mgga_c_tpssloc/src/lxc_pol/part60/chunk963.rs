//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 963/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk963<F: Float>(t19451: F, t8326: F, t28002: F, t1484: F, t7540: F, t22960: F, t25: F, t28447: F, t1530: F, t25373: F, t118480: F, t22986: F, t32814: F, t86873: F) -> (F, F, F, F, F, F, F, F, F) {
    let t126118 = F::new(2.0) * t19451 * t8326;
    let t126120 = F::new(4.0) * t28002 * t8326;
    let t126176 = t1484 * t7540;
    let t126177 = t22960 * t126176;
    let t126180 = t25 * t28447;
    let t126197 = t7540 * t1530;
    let t126198 = t25373 * t126197;
    let t126226 = F::new(0.15352717957250113407e0) * t118480;
    let t126229 = F::new(0.6579736267392905746e-1) * t22986 * t86873 * t32814;
    (t126118, t126120, t126176, t126177, t126180, t126197, t126198, t126226, t126229)
}
