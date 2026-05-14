//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 636/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk636<F: Float>(t22690: F, t6968: F, t22642: F, t268: F, t534: F, t6559: F) -> (F, F, F) {
    let t22691 = t22690 * t6968;
    let t22692 = t22642 * t22691;
    let t22693 = 0.82246703342411321824e-2 * t22692;
    let t22704 = t6559 * t534 * t268;
    (t22692, t22693, t22704)
}
