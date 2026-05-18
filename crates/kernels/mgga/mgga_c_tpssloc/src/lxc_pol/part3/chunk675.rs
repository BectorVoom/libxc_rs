//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 675/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk675<F: Float>(t3426: F, t461: F, t221: F, t456: F, t1176: F, t135: F) -> (F, F, F) {
    let t3427 = t3426 * t461;
    let t3428 = t221 * t3427;
    let t3430 = F::new(0.18518518518518518518e-3) * t456 * t3428;
    let t3431 = t135 * t1176;
    (t3428, t3430, t3431)
}
