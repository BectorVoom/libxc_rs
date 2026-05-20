//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2345/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2345<F: Float>(t13012: F, t20927: F, t13005: F, t41144: F, t41155: F, t41156: F, t41185: F, t41190: F, t46764: F, t46769: F, t46838: F, t59138: F, t59140: F, t68010: F) -> F {
    let t68073 = t13012 * t20927;
    let t68077 = -F::cast_from(0.59999999999999999996e-1_f64) * t13005 * t46838 * t68010 - F::cast_from(0.19999999999999999999e-1_f64) * t41144 + t41155 + F::cast_from(0.56172839506172839504e-1_f64) * t41156 - t41185 + F::cast_from(0.3287037037037037037e-1_f64) * t41190 - F::cast_from(0.59999999999999999998e-1_f64) * t46764 + t46769 - F::cast_from(0.34999999999999999998e-1_f64) * t68073 - F::cast_from(0.74999999999999999997e-2_f64) * t59138 - F::cast_from(0.34999999999999999998e-1_f64) * t59140;
    t68077
}
