//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 573/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk573<F: Float>(t2904: F, t315: F, t2764: F, t2822: F, t941: F) -> (F, F, F, F, F) {
    let t2905 = t315 * t2904;
    let t2912 = F::new(0.40256666666666666667e0) * t2764;
    let t2919 = F::new(0.137975e0) * t2822;
    let t2928 = t941 * t941;
    let t2929 = F::new(1.0) / t2928;
    (t2905, t2912, t2919, t2928, t2929)
}
