//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1056/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1056<F: Float>(t22228: F, t3403: F, t1164: F, t1147: F, t1156: F, t21938: F, t11282: F, t21906: F, t11285: F, t4869: F, t6102: F, t21726: F, t21728: F, t21730: F, t21732: F, t21897: F, t21901: F, t21990: F, t21993: F) -> (F, F, F, F, F) {
    let t22229 = t22228 * t3403;
    let t22231 = F::new(0.10389515463408878255e3) * t1164 * t22229;
    let t22233 = t1147 * t21938 * t1156;
    let t22235 = F::new(0.5848223622634646207e0) * t1164 * t22233;
    let t22236 = t11282 * t21906;
    let t22237 = t22236 * t11285;
    let t22239 = F::new(0.10254018858216406658e4) * t1164 * t22237;
    let t22241 = F::new(0.17544670867903938621e1) * t4869 * t6102;
    let t22242 = t22231 - t22235 - t22239 + t21726 - t21897 + t21901 - t21730 - t22241 - t21728 - t21990 + t21732 + t21993;
    (t22231, t22235, t22239, t22241, t22242)
}
