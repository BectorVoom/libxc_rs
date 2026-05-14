//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1201/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201<F: Float>(t10633: F, t2940: F, t10629: F, t2932: F, t41827: F, t959: F, t10619: F, t300: F, t961: F, t10957: F, t3053: F, t271: F, t2770: F, t41666: F, t10321: F, t1041: F, t248: F, t3051: F) -> (F, F, F, F, F, F, F) {
    let t42276 = 0.4101607543286562663e4 * t2940 * t10633;
    let t42280 = 0.6233709278045326953e3 * t959 * t10629 * t41827 * t2932;
    let t42281 = t300 * t10619;
    let t42283 = 0.23392894490538584828e1 * t42281 * t961;
    let t42303 = t10957 * t3053;
    let t42308 = 1.0 / t271 / t2770;
    let t42309 = t42308 * t41666;
    let t42322 = t1041 * t248 * t3051 * t10321;
    (t42276, t42280, t42283, t42303, t42308, t42309, t42322)
}
