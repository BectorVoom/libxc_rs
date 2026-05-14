//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1070/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1070<F: Float>(t41654: F, t10969: F, t154: F, t2769: F, t2289: F, t2903: F, t2928: F, t315: F, t10213: F, t241: F, t270: F, t276: F, t39267: F, t273: F, t242: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41655 = 0.18467901234567901234e0 * t41654;
    let t41664 = t154 * t10969;
    let t41665 = t2769 * t2769;
    let t41666 = 1.0 / t41665;
    let t41687 = 1.0 / t2769 / t2289;
    let t41741 = 0.96141975308641975307e-1 * t41654;
    let t41825 = 1.0 / t2928 / t2903;
    let t41826 = t315 * t41825;
    let t41880 = t241 * t10213;
    let t41904 = 280.0 / 81.0 * t41654;
    let t41935 = 1.0 / t276 / t39267 / t270 / 96.0;
    let t41942 = f64::powf(t273, -0.25e1);
    let t41959 = 0.31310740740740740741e1 * t41654;
    let t41961 = t281 * t242 * t283;
    (t41655, t41664, t41666, t41687, t41741, t41825, t41826, t41880, t41904, t41935, t41942, t41959, t41961)
}
