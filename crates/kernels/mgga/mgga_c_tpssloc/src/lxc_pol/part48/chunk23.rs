//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 23/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk23<F: Float>(t59: F, t63: F, t39: F, t44: F, t51: F, t56: F, t33: F) -> (F, F, F, F) {
    let t64 = t59 * t63;
    let t65 = t39 * t44 + t51 * t56 - t64;
    let t66 = t33 * t65;
    let t67 = M_CBRT3;
    (t64, t65, t66, t67)
}
