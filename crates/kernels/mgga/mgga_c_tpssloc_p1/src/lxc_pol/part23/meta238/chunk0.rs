//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 892/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk892<F: Float>(t5723: F, t699: F, t5769: F, t942: F, t5737: F, t923: F, t2932: F, t5790: F, t10632: F, t5774: F, t2844: F, t5726: F) -> (F, F, F, F, F, F) {
    let t17290 = t699 * t5723;
    let t17355 = t5769 * t942;
    let t17428 = t5737 * t923;
    let t17492 = t5790 * t2932;
    let t17499 = t5774 * t10632;
    let t17520 = t5726 * t2844;
    (t17290, t17355, t17428, t17492, t17499, t17520)
}
