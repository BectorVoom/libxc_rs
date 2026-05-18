//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 917/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk917<F: Float>(t2010: F, t22716: F, t154: F, t591: F, t6896: F) -> (F, F, F, F) {
    let t22717 = t22716 * t2010;
    let t22718 = F::new(0.63969658155208805863e-1) * t22717;
    let t22723 = t591 * t154;
    let t22724 = t22723 * t6896;
    (t22717, t22718, t22723, t22724)
}
