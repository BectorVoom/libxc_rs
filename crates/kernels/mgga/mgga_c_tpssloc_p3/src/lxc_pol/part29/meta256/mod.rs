//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1192;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1193;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1194;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta256<F: Float>(t1351: F, t562: F, t550: F, t6976: F, t1992: F, t1372: F, t1998: F, t214: F, t1985: F, t1338: F, t2006: F, t1352: F, t553: F, t6955: F, t1332: F, t1336: F, t2013: F, t544: F, t6967: F, t6971: F, t6975: F, t1378: F, t1375: F, t1386: F, t2016: F, t3758: F, t3882: F, t568: F, t6885: F, t6893: F, t6900: F, t6904: F, t6909: F, t6911: F, t6956: F, t6958: F, t6963: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6978, t6979, t6980, t6982, t6983, t6984, t6987) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1192::<F>(t1351, t562, t550, t6976, t1992, t1372, t1998, t214, t1985, t1338, t2006);
        let (t6988, t6990, t6992) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1193::<F>(t1352, t6987, t553, t6955, t1332, t1336, t2013, t544, t6967, t6971, t6975, t6980, t6984);
        let t6993 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1194::<F>(t1378, t6992);
        let t6995 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1195::<F>(t1375, t1386, t2016, t3758, t3882, t568, t6885, t6893, t6900, t6904, t6909, t6911, t6956, t6958, t6963, t6993);
    (t6978, t6979, t6982, t6983, t6987, t6988, t6990, t6992, t6993, t6995)
}
