//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2277;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta696<F: Float>(t3500: F, t3503: F, t65539: F, t1210: F, t15734: F, t5005: F, t19047: F, t3572: F, t11818: F, t248: F, t3506: F, t6225: F, t11539: F, t1174: F, t18211: F, t3540: F, t6170: F, t19015: F, t3577: F, t45124: F, t6158: F, t15730: F, t5002: F, t1226: F, t18573: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t65541, t65545, t65552, t65554, t65558) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2277::<F>(t3500, t3503, t65539, t1210, t15734, t5005, t19047, t3572, t11818, t248, t3506, t6225);
        let (t65567, t65581, t65598, t65600, t65605, t65607) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2278::<F>(t11539, t1174, t18211, t3540, t6170, t19015, t3577, t45124, t6158, t15730, t5002, t1226, t18573);
    (t65541, t65545, t65552, t65554, t65558, t65567, t65581, t65598, t65600, t65605, t65607)
}
