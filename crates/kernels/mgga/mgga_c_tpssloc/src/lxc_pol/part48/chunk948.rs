//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 948/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk948<F: Float>(t112984: F, t112988: F, t112990: F, t112992: F, t112995: F, t114649: F, t114655: F, t114659: F, t114663: F, t114666: F, t2633: F, t2684: F, t31394: F, t812: F, t829: F) -> F {
    let t114668 = -F::new(2.0) * t812 * t114649 * t829 - t812 * t31394 * t2684 + F::new(2.0) * t812 * t114655 * t2633 + F::new(0.76763589786250567036e-1) * t114659 - F::new(0.16449340668482264365e-1) * t114663 + F::new(0.16449340668482264365e-1) * t114666 + t112984 + t112988 + t112990 - t112992 + t112995;
    t114668
}
