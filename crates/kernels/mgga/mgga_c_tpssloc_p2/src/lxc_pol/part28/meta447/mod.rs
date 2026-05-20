//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1633;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1634;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1635;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta447<F: Float>(t23917: F, t510: F, t1266: F, t7056: F, t671: F, t7156: F, t111: F, t7039: F, t2035: F, t2319: F, t2095: F, t22578: F, t22584: F, t7170: F, t12734: F, t1393: F, t1983: F, t2036: F, t2040: F, t2079: F, t2314: F, t2323: F, t2364: F, t23909: F, t3652: F, t3929: F, t4034: F, t652: F, t672: F, t7040: F, t7042: F, t7050: F, t7057: F, t7061: F, t7166: F, t9348: F, t2094: F, t531: F, t22596: F, t7025: F, t9239: F, t33: F, t625: F, t2240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23918, t23929, t23933, t23938) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1633::<F>(t23917, t510, t1266, t7056, t671, t7156, t111, t7039);
        let (t23941, t23951, t23953, t23956) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1634::<F>(t2035, t2319, t2095, t22578, t22584, t7170, t1266, t12734, t1393, t1983, t2036, t2040, t2079, t2314, t2323, t2364, t23909, t23918, t23929, t23933, t23938, t3652, t3929, t4034, t510, t652, t672, t7040, t7042, t7050, t7057, t7061, t7166, t9348);
        let (t23958, t23963) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1635::<F>(t2094, t531, t22596, t7025, t9239);
        let (t23966, t23967) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1636::<F>(t33, t625, t2240);
    (t23918, t23929, t23933, t23938, t23941, t23951, t23953, t23956, t23958, t23963, t23966, t23967)
}
