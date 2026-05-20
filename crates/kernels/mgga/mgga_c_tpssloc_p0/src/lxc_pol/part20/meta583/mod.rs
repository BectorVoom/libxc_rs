//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2152;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta583<F: Float>(t1058: F, t3068: F, t3087: F, t363: F, t11065: F, t42387: F, t10250: F, t2970: F, t973: F, t10195: F, t10231: F, t1005: F, t10375: F, t10475: F, t42342: F, t42345: F, t2770: F, t283: F, t10309: F, t1041: F, t10457: F, t248: F, t10444: F, t354: F, t364: F, t372: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43358, t43361, t43374, t43377, t43382) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2152::<F>(t1058, t3068, t3087, t363, t11065, t42387, t10250, t2970, t973, t10195, t10231, t1005, t10375);
        let (t43385, t43398, t43406, t43410) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2153::<F>(t10475, t42342, t42345, t2770, t283, t10309, t1041, t10457, t248, t10444, t354, t364, t372);
    (t43358, t43361, t43374, t43377, t43382, t43385, t43398, t43406, t43410)
}
