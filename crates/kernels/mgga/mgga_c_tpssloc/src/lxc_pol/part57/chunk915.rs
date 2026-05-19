//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 915/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk915<F: Float>(t2006: F, t213: F, t225: F, t22573: F, t8449: F, t2627: F, t8543: F, t23030: F, t31381: F, t22690: F, t23171: F, t31376: F) -> (F, F, F, F, F) {
    let t114285 = t213 * t2006 * t225;
    let t114360 = t8449 * t22573;
    let t114655 = t2627 * t8543;
    let t114672 = t23030 * t31381;
    let t114673 = F::cast_from(0.26044789391763585244e-1_f64) * t114672;
    let t114688 = t23171 * t22690 * t31376;
    (t114285, t114360, t114655, t114673, t114688)
}
