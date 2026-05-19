//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 716/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk716<F: Float>(t205: F, t9558: F, t59: F, t8705: F, t207: F, t215: F, t2570: F, t782: F, t2690: F, t154: F, t2588: F, t21: F) -> (F, F, F, F, F, F, F) {
    let t9559 = t205 * t9558;
    let t9569 = t59 * t8705;
    let t9572 = F::cast_from(0.28086419753086419752e-1_f64) * t9569 * t207 * t215;
    let t9573 = t782 * t2570;
    let t9576 = t59 * t2690;
    let t9577 = t9576 * t154;
    let t9579 = F::cast_from(0.99999999999999999997e-2_f64) * t9577 * t2588;
    let t9580 = t59 * t21;
    (t9559, t9569, t9572, t9573, t9577, t9579, t9580)
}
