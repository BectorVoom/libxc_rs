//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1049;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1050;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta284<F: Float>(t12156: F, t210: F, t214: F, t1307: F, t213: F, t221: F, t3719: F, t116: F, t547: F, t212: F, t2586: F, t12012: F, t535: F, t9534: F, t9538: F, t12188: F, t12190: F, t12194: F, t12196: F, t12197: F, t12200: F, t12205: F, t12209: F, t12212: F, t12215: F, t1315: F, t5195: F, t225: F, t3792: F, t3850: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12217, t12220, t12222, t12225, t12226, t12227, t12228, t12231) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1049::<F>(t12156, t210, t214, t1307, t213, t221, t3719, t116, t547, t212, t2586, t12012);
        let t12237 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1050::<F>(t535, t9534, t9538, t12188, t12190, t12194, t12196, t12197, t12200, t12205, t12209, t12212, t12215, t12217, t12222, t12228, t12231, t1315, t5195);
        let (t12238, t12240) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1051::<F>(t12237, t225, t3792, t3850);
    (t12217, t12220, t12222, t12225, t12226, t12227, t12231, t12237, t12238, t12240)
}
