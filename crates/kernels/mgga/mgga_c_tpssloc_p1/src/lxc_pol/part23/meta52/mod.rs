//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta52 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk328;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk329;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk330;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta52<F: Float>(t1294: F, t763: F, t532: F, t571: F, t514: F, t517: F, t215: F, t535: F, t782: F, t154: F, t547: F, t205: F, t792: F, t795: F, t541: F, t801: F, t544: F, t68: F, t551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1296, t1297, t1298, t1302, t1313, t1314) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk328::<F>(t1294, t763, t532, t571, t514, t517, t215, t535, t782, t154, t547);
        let t1315 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk329::<F>(t1314, t205);
        let (t1322, t1327, t1336) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk330::<F>(t535, t792, t795, t541, t801, t544, t68);
        let (t1337, t1338) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk331::<F>(t551);
    (t1296, t1297, t1298, t1302, t1313, t1314, t1315, t1322, t1327, t1336, t1337, t1338)
}
