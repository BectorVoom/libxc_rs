//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 888/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk888<F: Float>(t32077: F, t32107: F, t532: F, t8803: F, t6879: F, t225: F, t8789: F, t31570: F, t31616: F, t31624: F, t1338: F, t8788: F) -> (F, F, F, F, F, F, F, F) {
    let t32108 = t32077 + t32107;
    let t32110 = t532 * t8803;
    let t32111 = t32110 * t6879;
    let t32120 = t8789 * t225;
    let t32127 = F::new(0.16449340668482264365e-1) * t31570;
    let t32130 = F::new(0.76763589786250567037e-1) * t31616;
    let t32132 = F::new(0.16449340668482264365e-1) * t31624;
    let t32136 = t1338 * t8788;
    (t32108, t32110, t32111, t32120, t32127, t32130, t32132, t32136)
}
