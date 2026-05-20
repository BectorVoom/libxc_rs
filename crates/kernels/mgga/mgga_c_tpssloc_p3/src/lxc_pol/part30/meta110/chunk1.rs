//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 675/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk675<F: Float>(t116: F, t206: F, t212: F, t2586: F, t225: F, t799: F) -> (F, F, F, F) {
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = F::cast_from(0.83333333333333333332e-3_f64) * t2586 * t2588;
    let t2597 = t799 * t225;
    (t2587, t2588, t2590, t2597)
}
