//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1958;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1959;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1960;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta479<F: Float>(t25: F, t12061: F, t1408: F, t2: F, t3664: F, t584: F, t606: F, t16: F, t2249: F, t3665: F, t5134: F, t5137: F, t514: F, zeta_threshold: F, t28: F, t12072: F, t1649: F, t3672: F, t1081: F, t3231: F, t3673: F, t5142: F, t5145: F, t517: F, t157: F) -> (F, F, F, F, F, F, F) {
        let (t15937, t15940, t15941, t15951) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1958::<F>(t25, t12061, t1408, t2, t3664, t584, t606, t16, t2249, t3665, t5134, t5137, t514, zeta_threshold);
        let (t15952, t15955, t15956, t15966) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1959::<F>(t28, t12072, t1649, t2, t3672, t1081, t584, t16, t3231, t3673, t5142, t5145, t517, zeta_threshold);
        let t15968 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1960::<F>(t157, t15951, t15966);
    (t15937, t15940, t15941, t15952, t15955, t15956, t15968)
}
