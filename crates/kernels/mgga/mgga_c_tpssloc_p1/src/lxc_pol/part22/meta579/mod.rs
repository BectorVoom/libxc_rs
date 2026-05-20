//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta579<F: Float>(t11715: F, t42341: F, t44696: F, t11721: F, t23508: F, t11714: F, t476: F, t3508: F, t3502: F, t1209: F, t3242: F, t415: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44698, t44701, t44722, t44724, t44725, t44726, t44753, t44754, t44785, t44827) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2088::<F>(t11715, t42341, t44696, t11721, t23508, t11714, t476, t3508, t3502, t1209, t3242, t415);
    (t44698, t44701, t44722, t44724, t44725, t44726, t44753, t44754, t44785, t44827)
}
