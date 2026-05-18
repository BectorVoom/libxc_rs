//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 426/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk426<F: Float>(t7603: F, t8743: F, t27: F, t3819: F, t8747: F, t3851: F, t8751: F, t7599: F, t8754: F, t8645: F, t3839: F, t8641: F) -> (F, F, F, F, F, F) {
    let t8759 = t7603 * t8743;
    let t8761 = t3819 * t27;
    let t8762 = t8761 * t8747;
    let t8764 = t3851 * t27;
    let t8765 = t8764 * t8751;
    let t8767 = t7599 * t8754;
    let t8769 = t3851 * t8645;
    let t8771 = t3839 * t8641;
    (t8759, t8762, t8765, t8767, t8769, t8771)
}
