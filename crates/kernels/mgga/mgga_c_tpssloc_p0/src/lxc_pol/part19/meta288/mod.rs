//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1056;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta288<F: Float>(t3777: F, t3798: F, t1354: F, t1307: F, t3719: F, t3870: F, t820: F, t12189: F, t1329: F, t3726: F, t3770: F, t119: F, t12012: F, t210: F, t12211: F, t3766: F, t12156: F, t1358: F, t3774: F, t1333: F, t3862: F, t10022: F, t248: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12300, t12301, t12303, t12305, t12308, t12310, t12313) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1056::<F>(t3777, t3798, t1354, t1307, t3719, t3870, t820, t12189, t1329, t3726, t3770, t119, t12012, t210);
        let (t12317, t12320, t12323, t12325, t12328) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1057::<F>(t12211, t3766, t119, t12156, t210, t1358, t3774, t1333, t3862, t10022, t248, t557);
    (t12300, t12301, t12303, t12305, t12308, t12310, t12313, t12317, t12320, t12323, t12325, t12328)
}
