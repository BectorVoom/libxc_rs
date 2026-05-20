//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta602<F: Float>(t11721: F, t23508: F, t1009: F, t11598: F, t1243: F, t11714: F, t476: F, t42341: F, t44696: F, t3508: F, t3502: F, t1209: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44701, t44706, t44707, t44722, t44724, t44725, t44726, t44753, t44754, t44785) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2182::<F>(t11721, t23508, t1009, t11598, t1243, t11714, t476, t42341, t44696, t3508, t3502, t1209);
    (t44701, t44706, t44707, t44722, t44724, t44725, t44726, t44753, t44754, t44785)
}
