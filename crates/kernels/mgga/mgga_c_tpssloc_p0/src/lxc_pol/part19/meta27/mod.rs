//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta27 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk206;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk207;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk208;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk209;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk210;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk211;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk212;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk213;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta27<F: Float>(t144: F, t225: F, t523: F, t525: F, t533: F, t68: F, t236: F, t544: F, t532: F, t242: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t546 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk206::<F>(t144, t225, t523, t525);
        let (t547, t548) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk207::<F>(t533, t68);
        let t550 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk208::<F>(t546, t548);
        let (t551, t552, t553) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk209::<F>(t550, t68);
        let t554 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk210::<F>(t236, t553);
        let t555 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk211::<F>(t544, t554);
        let t556 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk212::<F>(t532);
        let t557 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk213::<F>(t556);
        let t559 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk214::<F>(t242, t248, t557);
    (t546, t547, t548, t550, t551, t552, t553, t554, t555, t556, t557, t559)
}
