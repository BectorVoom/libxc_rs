//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1693;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1694;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1695;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta363<F: Float>(t1409: F, t9330: F, t2298: F, t3966: F, t12595: F, t12598: F, t12606: F, t2244: F, t2250: F, t4007: F, t4012: F, t607: F, t634: F, t638: F, t72: F, t1410: F, t2283: F, t1426: F, t2251: F, t3997: F, t608: F, t1411: F, t1434: F, t2245: F, t2252: F, t2284: F, t2304: F, t3971: F, t3976: F, t4018: F, t629: F, t642: F, t66: F, t80: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12609, t12612, t12619) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1693::<F>(t1409, t9330, t2298, t3966, t12595, t12598, t12606, t2244, t2250, t4007, t4012, t607, t634, t638);
        let (t12620, t12623, t12630, t12633, t12636, t12645) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1694::<F>(t12619, t72, t1410, t2283, t1426, t2244, t2251, t3997, t608, t1411, t1434, t2245, t2252, t2284, t2304, t3971, t3976, t4018, t629, t642, t66, t80);
        let t12648 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1695::<F>(t1409, t2250);
        let (t12649, t12652) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1696::<F>(t12648, t65, t3966, t607);
    (t12609, t12612, t12620, t12623, t12630, t12633, t12636, t12645, t12648, t12649, t12652)
}
