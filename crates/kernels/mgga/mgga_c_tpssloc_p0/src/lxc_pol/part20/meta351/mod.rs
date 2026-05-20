//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1659;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta351<F: Float>(t12225: F, t12226: F, t2586: F, t12012: F, t210: F, t214: F, t535: F, t9534: F, t9538: F, t12188: F, t12190: F, t12194: F, t12196: F, t12197: F, t12200: F, t12205: F, t12209: F, t12212: F, t12215: F, t12217: F, t12222: F, t1315: F, t5195: F, t225: F, t3792: F, t3850: F) -> (F, F, F, F, F, F, F) {
        let (t12227, t12228, t12231, t12236, t12237) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1659::<F>(t12225, t12226, t2586, t12012, t210, t214, t535, t9534, t9538, t12188, t12190, t12194, t12196, t12197, t12200, t12205, t12209, t12212, t12215, t12217, t12222, t1315, t5195);
        let (t12238, t12240) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1660::<F>(t12237, t225, t3792, t3850);
    (t12227, t12228, t12231, t12236, t12237, t12238, t12240)
}
