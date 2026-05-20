//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1356/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1356<F: Float>(t10544: F, t2841: F, t888: F, t2840: F, t287: F, t275: F) -> (F, F, F, F) {
    let t10636 = F::cast_from(0.55403703703703703703e-1_f64) * t10544;
    let t10655 = t888 * t2841;
    let t10660 = F::new(1.0) / t2840 / t287;
    let t10661 = t275 * t10660;
    (t10636, t10655, t10660, t10661)
}
