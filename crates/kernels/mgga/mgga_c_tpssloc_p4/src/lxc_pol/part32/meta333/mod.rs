//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1365;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta333<F: Float>(t1307: F, t212: F, t12225: F, t2586: F, t535: F, t9534: F, t9538: F, t1337: F, t3792: F, t550: F, t1339: F, t836: F, t1336: F, t3777: F, t3789: F, t236: F, t3798: F, t12189: F, t1329: F, t1333: F, t3862: F, t10022: F, t248: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12228, t12236, t12248, t12250, t12282) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1365::<F>(t1307, t212, t12225, t2586, t535, t9534, t9538, t1337, t3792, t550, t1339, t836);
        let (t12283, t12286, t12289, t12300, t12308, t12325, t12328) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1366::<F>(t12282, t1336, t3777, t3789, t12248, t236, t3798, t12189, t1329, t1333, t3862, t10022, t248, t557);
    (t12228, t12236, t12248, t12250, t12283, t12286, t12289, t12300, t12308, t12325, t12328)
}
