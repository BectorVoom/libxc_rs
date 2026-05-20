//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1988;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta600<F: Float>(t22690: F, t3787: F, t22832: F, t3777: F, t1336: F, t6943: F, t836: F, t1995: F, t1999: F, t213: F, t39041: F, t557: F, t6546: F, t1365: F, t1878: F, t22813: F, t6924: F, t80782: F, t22794: F, t22843: F, t281: F, t6597: F, t154: F, t8705: F, t1887: F, t534: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t80798, t80816, t80820, t80826, t80827) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1988::<F>(t22690, t3787, t22832, t3777, t1336, t6943, t836, t1995, t1999, t213, t39041, t557, t6546);
        let (t80830, t80836, t80837, t80840, t80845, t80847) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1989::<F>(t1365, t1878, t22813, t6924, t80782, t22794, t22843, t281, t6597, t154, t8705, t1887, t534);
    (t80798, t80816, t80820, t80826, t80827, t80830, t80836, t80837, t80840, t80845, t80847)
}
