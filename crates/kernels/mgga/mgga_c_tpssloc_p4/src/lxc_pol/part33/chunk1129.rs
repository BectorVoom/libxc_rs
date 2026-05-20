//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1129/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1129<F: Float>(t6743: F, t7577: F, t7614: F, t968: F, t1920: F, t23384: F, t7604: F, t4640: F, t6754: F, t1611: F, t6764: F, t4603: F, t6717: F) -> (F, F, F, F, F, F) {
    let t25523 = t7577 * t6743;
    let t25529 = t968 * t7614;
    let t25530 = t1920 * t25529;
    let t25563 = t23384 * t7604;
    let t25577 = t4640 * t6754;
    let t25580 = t1611 * t6764;
    let t25598 = t6717 * t4603;
    (t25523, t25530, t25563, t25577, t25580, t25598)
}
