//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk934;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk935;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta146<F: Float>(t4395: F, t913: F, t893: F, t1556: F, t2844: F, t912: F, t2842: F, t2766: F, t2848: F, t4335: F, t4340: F, t4345: F, t4349: F, t1561: F, t923: F, t1569: F, t931: F, t2824: F, t2868: F, t2875: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4396, t4398, t4399, t4400, t4402, t4408) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk934::<F>(t4395, t913, t893, t1556, t2844, t912, t2842, t2766, t2848, t4335, t4340, t4345, t4349);
        let t4411 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk935::<F>(t1561, t923);
        let (t4416, t4433) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk936::<F>(t1569, t931, t2766, t2824, t2868, t2875, t4335, t4340, t4345, t4349, t4363, t4371, t4379, t4381, t4384, t4387, t4390, t4393);
    (t4396, t4398, t4399, t4400, t4402, t4408, t4411, t4416, t4433)
}
