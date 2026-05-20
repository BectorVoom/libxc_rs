//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 965/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk965<F: Float>(t24574: F, t7368: F, t2148: F, t3427: F, t2121: F, t225: F, t7319: F, t23598: F, t50: F, t131: F, t467: F, t3030: F, t461: F) -> (F, F, F, F, F) {
    let t24760 = t24574 * t7368;
    let t24771 = t3427 * t2148;
    let t24773 = F::cast_from(0.18277045187202515961e-2_f64) * t2121 * t24771;
    let t24788 = t7319 * t225;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    let t24813 = t461 * t3030;
    (t24760, t24773, t24788, t24812, t24813)
}
