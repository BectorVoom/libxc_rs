//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 814/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk814<F: Float>(t213: F, t776: F, t221: F, t2553: F, t59: F, t8705: F, t207: F, t215: F, t2570: F, t782: F, t2573: F, t2690: F) -> (F, F, F, F, F, F) {
    let t9564 = t213 * t776;
    let t9566 = t221 * t9564 * t2553;
    let t9569 = t59 * t8705;
    let t9572 = F::new(0.28086419753086419752e-1) * t9569 * t207 * t215;
    let t9573 = t782 * t2570;
    let t9574 = t9573 * t2573;
    let t9576 = t59 * t2690;
    (t9566, t9569, t9572, t9573, t9574, t9576)
}
