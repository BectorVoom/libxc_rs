//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 976/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk976<F: Float>(t28: F, t28447: F, t1649: F, t7540: F, t126197: F, t25927: F, t19451: F, t8327: F, t28002: F, t32677: F, t4028: F, t28237: F, t3701: F) -> (F, F, F, F, F, F, F) {
    let t126992 = t28 * t28447;
    let t127017 = t1649 * t7540;
    let t127030 = t25927 * t126197;
    let t127107 = F::new(2.0) * t19451 * t8327;
    let t127109 = F::new(4.0) * t28002 * t8327;
    let t127111 = F::new(4.0) * t4028 * t32677;
    let t127114 = t3701 * t28237;
    (t126992, t127017, t127030, t127107, t127109, t127111, t127114)
}
