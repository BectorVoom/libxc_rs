//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1934;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta654<F: Float>(t23097: F, t232: F, t67783: F, t815: F, t16888: F, t23146: F, t16969: F, t25146: F, t4236: F, t23053: F, t5614: F, t16859: F, t6614: F, t16673: F, t6613: F, t831: F, t28359: F, t838: F, t23069: F, t5572: F, t23062: F, t28383: F, t20986: F, t2628: F, t6605: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t98672, t98674, t98676, t98678, t98680, t98682) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1934::<F>(t23097, t232, t67783, t815, t16888, t23146, t16969, t25146, t4236, t23053, t5614, t16859, t6614);
        let (t98685, t98690, t98694, t98696, t98701) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1935::<F>(t16673, t6613, t831, t28359, t838, t23069, t5572, t23062, t28383, t20986, t2628, t6605, t828);
    (t98672, t98674, t98676, t98678, t98680, t98682, t98685, t98690, t98694, t98696, t98701)
}
