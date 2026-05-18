//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1236/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1236<F: Float>(t24815: F, t6252: F, t24814: F, t24821: F, t24820: F, t5979: F, t7363: F, t7362: F, t5975: F, t29664: F, t493: F, t5971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29749 = t6252 * t24815;
    let t29750 = t24814 * t29749;
    let t29753 = t6252 * t24821;
    let t29754 = t24820 * t29753;
    let t29758 = t7363 * t5979;
    let t29759 = t7362 * t29758;
    let t29762 = t7363 * t5975;
    let t29763 = t7362 * t29762;
    let t29773 = t493 * t29664;
    let t29776 = t7363 * t5971;
    (t29749, t29750, t29753, t29754, t29758, t29759, t29762, t29763, t29773, t29776)
}
