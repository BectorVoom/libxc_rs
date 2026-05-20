//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1843;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta515<F: Float>(t25374: F, t25927: F, t1081: F, t1530: F, t28: F, t4303: F, t1649: F, t776: F, t868: F, t1877: F, t1915: F, t22959: F, t23290: F, t25013: F, t2522: F, t25354: F, t25358: F, t25372: F, t25397: F, t25892: F, t25898: F, t25901: F, t25905: F, t25921: F, t6666: F, t6670: F, t6841: F, t6848: F, t7541: F, t7649: F, t7656: F, t265: F, t504: F, t25882: F, t1409: F, t1972: F, t3966: F, t52: F, t607: F, t6856: F, t7664: F, t25890: F, t113: F, t2314: F, t24980: F, t24983: F, t24988: F, t24989: F, t24993: F, t24998: F, t24999: F, t25005: F, t25007: F, t25011: F, t4073: F, t4077: F, t6517: F, t652: F, t672: F, t7472: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
        let (t25928, t25930, t25934, t25938, t25945, t25949) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1843::<F>(t25374, t25927, t1081, t1530, t28, t4303, t1649, t776, t868, t1877, t1915, t22959, t23290, t25013, t2522, t25354, t25358, t25372, t25397, t25892, t25898, t25901, t25905, t25921, t6666, t6670, t6841, t6848, t7541, t7649, t7656);
        let (t25950, t25958, t25962) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1844::<F>(t28, t265, t504, t25882, t1409, t1972, t25949, t3966, t52, t607, t6856, t7664, t25890, t113, t2314, t24980, t24983, t24988, t24989, t24993, t24998, t24999, t25005, t25007, t25011, t4073, t4077, t6517, t652, t672, t7472, dens_threshold, rho1, zeta_threshold);
    (t25928, t25930, t25934, t25938, t25945, t25950, t25958, t25962)
}
