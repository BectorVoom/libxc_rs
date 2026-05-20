//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2428/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2428<F: Float>(t2403: F, t2830: F, t909: F, t9709: F, t2833: F, t2827: F, t10213: F, t241: F, t41654: F, t270: F, t276: F, t39267: F) -> (F, F, F, F, F, F, F) {
    let t41831 = t2403 * t2830;
    let t41863 = t9709 * t909;
    let t41870 = t2403 * t2833;
    let t41872 = t2403 * t2827;
    let t41880 = t241 * t10213;
    let t41904 = F::new(280.0) / F::new(81.0) * t41654;
    let t41935 = F::new(1.0) / t276 / t39267 / t270 / F::new(96.0);
    (t41831, t41863, t41870, t41872, t41880, t41904, t41935)
}
