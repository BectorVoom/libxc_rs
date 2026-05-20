//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk751;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk752;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta146<F: Float>(t3: F, t3931: F, t112: F, t1395: F, t111: F, t576: F, t1401: F, t2319: F, t2363: F, t577: F, t671: F, t89: F, t131: F, t2570: F, t205: F, t242: F, t2628: F, t812: F, t244: F, t67: F, t246: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3932, t3938, t3941, t3946, t4034) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk751::<F>(t3, t3931, t112, t1395, t111, t576, t1401, t2319, t2363, t577, t671, t89);
        let (t4126, t4127, t4177, t4178) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk752::<F>(t131, t2570, t205, t242, t2628, t812);
        let t4180 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk753::<F>(t244, t67, t246);
    (t3932, t3938, t3941, t3946, t4034, t4126, t4127, t4177, t4178, t4180)
}
