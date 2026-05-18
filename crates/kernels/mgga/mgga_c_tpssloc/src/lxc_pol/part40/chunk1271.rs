//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1271/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1271<F: Float>(t5493: F, t88: F, t89: F, t5456: F, t576: F, t2177: F, t2281: F, t2331: F, t626: F) -> (F, F, F, F, F) {
    let t28007 = t88 * t5493;
    let t28030 = t89 * t5493;
    let t28893 = t576 * t5456;
    let t29894 = F::new(11.0) / F::new(9.0) * t2281 * t2177;
    let t29895 = t626 * t2331;
    (t28007, t28030, t28893, t29894, t29895)
}
