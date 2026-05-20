//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1798/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1798<F: Float>(t25097: F, t81782: F, t81783: F, t1516: F, t81769: F, t23133: F, t4261: F, t25111: F, t25115: F, t87229: F, t23132: F, t4166: F) -> (F, F, F, F, F, F) {
    let t87328 = t81782 * t81783 * t25097;
    let t87330 = t81769 * t1516;
    let t87332 = t23133 * t4261;
    let t87335 = t81782 * t81783 * t25111;
    let t87338 = t87229 * t81783 * t25115;
    let t87340 = t4166 * t23132;
    (t87328, t87330, t87332, t87335, t87338, t87340)
}
