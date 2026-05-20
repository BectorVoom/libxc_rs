//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1750;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1751;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1752;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1753;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta506<F: Float>(t111: F, t7786: F, t1268: F, t12725: F, t1458: F, t19456: F, t2039: F, t2314: F, t23938: F, t26114: F, t26117: F, t26967: F, t26977: F, t27170: F, t4028: F, t4072: F, t5113: F, t671: F, t7042: F, t7056: F, t7676: F, t7801: F, t1774: F, t1266: F, t1442: F, t1459: F, t2036: F, t2040: F, t2075: F, t4026: F, t4034: F, t4073: F, t4077: F, t5107: F, t574: F, t652: F, t672: F, t7040: F, t7156: F, t7787: F, t7802: F, t26895: F, t26982: F, t27183: F, t3: F, t112: F, t7945: F, t12524: F, t1401: F, t16521: F, t16524: F, t20173: F, t24462: F, t24465: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F, t7230: F, t7235: F, t7956: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t27188 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1750::<F>(t111, t7786);
        let t27215 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1751::<F>(t1268, t12725, t1458, t19456, t2039, t2314, t23938, t26114, t26117, t26967, t26977, t27170, t27188, t4028, t4072, t5113, t671, t7042, t7056, t7676, t7801);
        let (t27219, t27226, t27238) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1752::<F>(t1774, t7056, t1266, t7801, t12725, t1442, t1459, t2036, t2040, t2075, t2314, t23938, t27188, t27215, t4026, t4034, t4073, t4077, t5107, t574, t652, t672, t7040, t7042, t7156, t7787, t7802);
        let (t27240, t27241, t27254, t27273, t27276, t27281) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1753::<F>(t26895, t26982, t27183, t27238, t3, t112, t7945, t1458, t7056, t2039, t4072, t671, t7801);
        let t27286 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1754::<F>(t12524, t1401, t1458, t16521, t16524, t20173, t2039, t24462, t24465, t27170, t27240, t27254, t27273, t27276, t27281, t3938, t3941, t4072, t5371, t5376, t577, t671, t7056, t7230, t7235, t7801, t7956);
    (t27188, t27215, t27219, t27226, t27240, t27241, t27254, t27273, t27276, t27281, t27286)
}
