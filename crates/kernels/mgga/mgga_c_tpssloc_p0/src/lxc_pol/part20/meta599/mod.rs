//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta599<F: Float>(t1176: F, t697: F, t1184: F, t3447: F, t3451: F, t11579: F, t11589: F, t11168: F, t15402: F, t11159: F, t15419: F, t11584: F) -> (F, F, F, F, F, F, F) {
        let (t44583, t44584, t44586, t44589, t44592, t44595, t44602) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2179::<F>(t1176, t697, t1184, t3447, t3451, t11579, t11589, t11168, t15402, t11159, t15419, t11584);
    (t44583, t44584, t44586, t44589, t44592, t44595, t44602)
}
