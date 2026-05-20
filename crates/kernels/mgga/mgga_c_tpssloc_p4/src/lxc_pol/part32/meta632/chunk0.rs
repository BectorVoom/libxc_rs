//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2044/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2044<F: Float>(t13176: F, t6620: F, t25097: F, t81782: F, t81783: F, t1516: F, t81769: F, t23133: F, t4261: F, t25111: F, t25115: F, t87229: F) -> (F, F, F, F, F, F) {
    let t87321 = t13176 * t6620;
    let t87328 = t81782 * t81783 * t25097;
    let t87329 = F::cast_from(0.40372756094140390854e-3_f64) * t87328;
    let t87330 = t81769 * t1516;
    let t87331 = F::new(7.0) / F::new(288.0) * t87330;
    let t87332 = t23133 * t4261;
    let t87333 = F::new(7.0) / F::new(288.0) * t87332;
    let t87335 = t81782 * t81783 * t25111;
    let t87336 = F::cast_from(0.40372756094140390854e-3_f64) * t87335;
    let t87338 = t87229 * t81783 * t25115;
    (t87321, t87329, t87331, t87333, t87336, t87338)
}
