//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2274;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2275;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2276;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2277;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2278;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta652<F: Float>(t1268: F, t86604: F, t1873: F, t55934: F, t12725: F, t6534: F, t55962: F, t19456: F, t4072: F, t649: F, t26114: F, t12813: F, t88: F, t22479: F, t4028: F, t1458: F, t2363: F, t24999: F, t83935: F, t90351: F, t90352: F, t90355: F, t2311: F, t7676: F, t7467: F, t9348: F, t45632: F, t111: F, t26097: F, t12734: F, t2314: F, t26135: F, t12739: F, t5113: F, t22461: F, t26103: F, t6517: F, t671: F, t90041: F, t90044: F, t1983: F, t23857: F, t7753: F, t24991: F, t6876: F, t12728: F, t1459: F, t16503: F, t1976: F, t1980: F, t23829: F, t24980: F, t4034: F, t4037: F, t574: F, t652: F, t90034: F, t90036: F, t90038: F, t90040: F, t90051: F, t90059: F, t90062: F, t90064: F, t90068: F, t25992: F, t22592: F, t7685: F, t22948: F, t5161: F, t1845: F, t3914: F, t26161: F, t26162: F, t24994: F, t6875: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90361, t90363, t90365, t90367, t90369, t90370, t90372, t90374, t90375) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2274::<F>(t1268, t86604, t1873, t55934, t12725, t6534, t55962, t19456, t4072, t649, t26114, t12813, t88);
        let t90380 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2275::<F>(t1873, t90375, t22479, t4028, t1458, t2363, t24999, t83935, t90351, t90352, t90355, t90361, t90363, t90365, t90367, t90369, t90372, t90374);
        let (t90381, t90383, t90385, t90387, t90399, t90400, t90404, t90406) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2276::<F>(t1458, t2311, t1873, t22479, t7676, t7467, t9348, t45632, t111, t26097, t12734, t2314, t26135);
        let t90411 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2277::<F>(t12739, t7467, t26135, t5113, t12813, t1458, t22461, t26103, t4072, t6517, t671, t90041, t90044, t90383, t90385, t90387, t90399, t90400, t90404, t90406);
        let t90422 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2278::<F>(t1983, t23857, t7753, t24991, t6876, t12728, t1458, t1459, t16503, t1976, t1980, t23829, t24980, t26103, t4034, t4037, t574, t652, t90034, t90036, t90038, t90040, t90041, t90044, t90051, t90059, t90062, t90064, t90068, t90380, t90411);
        let (t90428, t90434, t90436, t90440, t90442) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2279::<F>(t25992, t6876, t22592, t7685, t1983, t22948, t5161, t1845, t3914, t26161, t26162, t24994, t6875);
    (t90370, t90381, t90400, t90422, t90428, t90434, t90436, t90440, t90442)
}
