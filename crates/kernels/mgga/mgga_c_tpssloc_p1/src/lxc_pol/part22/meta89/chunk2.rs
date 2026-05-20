//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 620/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk620<F: Float>(t107: F, t2281: F, t626: F, t667: F, t106: F, t655: F) -> (F, F, F) {
    let t2327 = F::new(11.0) / F::new(9.0) * t2281 * t107;
    let t2328 = t626 * t667;
    let t2331 = F::new(1.0) / t655 / t106;
    (t2327, t2328, t2331)
}
