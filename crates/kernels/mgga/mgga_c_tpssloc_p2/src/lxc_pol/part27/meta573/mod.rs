//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2019;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta573<F: Float>(t22832: F, t3777: F, t1336: F, t6943: F, t836: F, t3809: F, t1995: F, t1999: F, t213: F, t39041: F, t557: F, t6546: F, t3766: F, t1365: F, t1878: F, t22813: F, t6924: F, t80782: F, t22794: F, t22843: F, t281: F, t6597: F, t1361: F, t22690: F, t3734: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t80816, t80820, t80821, t80826, t80827) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2019::<F>(t22832, t3777, t1336, t6943, t836, t3809, t1995, t1999, t213, t39041, t557, t6546);
        let (t80828, t80830, t80836, t80837, t80840, t80843) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2020::<F>(t3766, t80827, t1365, t1878, t22813, t6924, t80782, t22794, t22843, t281, t6597, t1361, t22690, t3734);
    (t80816, t80820, t80821, t80826, t80827, t80828, t80830, t80836, t80837, t80840, t80843)
}
