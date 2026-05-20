//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1315;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta289<F: Float>(t225: F, t2711: F, t2594: F, t2690: F, t841: F, t812: F, t849: F, t2697: F, t2707: F, t241: F, t6589: F, t67: F, t2613: F, t68: F) -> (F, F, F, F, F, F, F) {
        let (t9590, t9593, t9601, t9602, t9604, t9607) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1315::<F>(t225, t2711, t2594, t2690, t841, t812, t849, t2697, t2707, t241, t6589, t67);
        let t9612 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1316::<F>(t2613, t68);
    (t9590, t9593, t9601, t9602, t9604, t9607, t9612)
}
