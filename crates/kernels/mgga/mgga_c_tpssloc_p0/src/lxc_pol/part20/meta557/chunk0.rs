//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2113/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2113<F: Float>(t10319: F, t699: F, t10313: F, t41654: F, t270: F, t276: F, t39267: F, t273: F, t242: F, t281: F, t283: F, t2853: F, t2860: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41887 = t699 * t10319;
    let t41889 = t699 * t10313;
    let t41904 = F::new(280.0) / F::new(81.0) * t41654;
    let t41935 = F::new(1.0) / t276 / t39267 / t270 / F::new(96.0);
    let t41942 = F::powf(t273, -F::new(0.25e1));
    let t41959 = F::cast_from(0.31310740740740740741e1_f64) * t41654;
    let t41961 = t281 * t242 * t283;
    let t41962 = F::cast_from(0.13490888888888888889e1_f64) * t41961;
    let t41981 = t2853 * t2860;
    (t41887, t41889, t41904, t41935, t41942, t41959, t41961, t41962, t41981)
}
