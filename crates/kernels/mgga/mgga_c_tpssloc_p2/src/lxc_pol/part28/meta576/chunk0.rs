//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1858/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1858<F: Float>(t23097: F, t4234: F, t776: F, t815: F, t13176: F, t6620: F, t849: F, t25097: F, t81782: F, t81783: F, t1516: F, t81769: F) -> (F, F, F, F) {
    let t87316 = t23097 * t815 * t4234 * t776;
    let t87321 = t13176 * t6620;
    let t87322 = t87321 * t849;
    let t87328 = t81782 * t81783 * t25097;
    let t87330 = t81769 * t1516;
    (t87316, t87322, t87328, t87330)
}
