//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta530<F: Float>(t24615: F, t5059: F, t7300: F, t5088: F, t7301: F, t2144: F, t4940: F, t1238: F, t24575: F, t24577: F, t24587: F, t27383: F, t27389: F, t27392: F, t27396: F, t27401: F, t27403: F, t27406: F, t3593: F, t498: F, t7283: F, t7291: F, t7303: F, t8061: F) -> (F, F, F, F, F, F) {
        let (t27411, t27412, t27415, t27416, t27419, t27421) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1867::<F>(t24615, t5059, t7300, t5088, t7301, t2144, t4940, t1238, t24575, t24577, t24587, t27383, t27389, t27392, t27396, t27401, t27403, t27406, t3593, t498, t7283, t7291, t7303, t8061);
    (t27411, t27412, t27415, t27416, t27419, t27421)
}
