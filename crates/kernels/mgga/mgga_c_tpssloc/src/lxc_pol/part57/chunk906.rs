//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 906/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk906<F: Float>(t112: F, t29395: F, t12461: F, t7939: F, t2752: F, t29105: F, t225: F, t29095: F, t29099: F, t10109: F, t7841: F, t29071: F) -> (F, F, F, F, F, F, F) {
    let t100996 = t29395 * t112;
    let t101138 = t7939 * t12461;
    let t101226 = t29105 * t2752;
    let t101355 = t29095 * t225;
    let t101509 = t29099 * t225;
    let t101551 = t10109 * t7841;
    let t101593 = t29071 * t225;
    (t100996, t101138, t101226, t101355, t101509, t101551, t101593)
}
