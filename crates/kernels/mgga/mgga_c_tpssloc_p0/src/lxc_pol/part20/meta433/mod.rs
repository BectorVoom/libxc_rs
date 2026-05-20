//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1853;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta433<F: Float>(t4677: F, t4684: F, t14506: F, t3185: F, t1932: F, t3120: F, t360: F, t1629: F, t1625: F, t3040: F, t3201: F, t6739: F, t14526: F, t383: F, t1022: F, t4657: F, t1060: F, t3188: F, t1057: F, t14205: F, t11054: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14615, t14618, t14622, t14623, t14626, t14627, t14630) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1853::<F>(t4677, t4684, t14506, t3185, t1932, t3120, t360, t1629, t1625, t3040, t3201, t6739);
        let (t14631, t14640, t14645, t14648, t14651, t14654) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1854::<F>(t14630, t1629, t14526, t383, t1022, t4657, t1060, t14626, t3188, t1057, t14205, t11054);
    (t14615, t14618, t14622, t14623, t14627, t14630, t14631, t14640, t14645, t14648, t14651, t14654)
}
