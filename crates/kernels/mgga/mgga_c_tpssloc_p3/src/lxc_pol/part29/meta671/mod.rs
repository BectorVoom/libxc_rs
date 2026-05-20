//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2244;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2245;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2246;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2247;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta671<F: Float>(t1361: F, t16153: F, t26288: F, t1339: F, t16206: F, t6936: F, t1825: F, t22827: F, t3719: F, t1307: F, t7708: F, t80840: F, t90787: F, t26245: F, t80783: F, t80870: F, t80872: F, t91304: F, t91305: F, t91311: F, t91312: F, t91314: F, t91317: F, t91319: F, t91321: F, t91323: F, t91328: F, t91330: F, t22897: F, t6925: F, t12369: F, t1351: F, t26243: F, t26302: F, t80958: F, t22779: F, t26323: F, t1336: F, t242: F, t80901: F, t16235: F, t5303: F, t80820: F, t16356: F, t6916: F, t16018: F, t1998: F, t236: F, t6926: F, t54153: F, t550: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91333, t91336, t91340, t91344) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2244::<F>(t1361, t16153, t26288, t1339, t16206, t6936, t1825, t22827, t3719, t1307, t7708, t80840, t90787);
        let t91348 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2245::<F>(t91344, t26245, t80783, t80870, t80872, t91304, t91305, t91311, t91312, t91314, t91317, t91319, t91321, t91323, t91328, t91330, t91333, t91336, t91340);
        let (t91354, t91357, t91359, t91361) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2246::<F>(t22897, t6925, t12369, t1351, t26243, t26302, t80958, t22779, t26323, t1336, t242, t80901);
        let (t91362, t91365, t91366, t91370, t91374) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2247::<F>(t16235, t91361, t5303, t80820, t16356, t6916, t16018, t1998, t236, t6926, t1339, t54153, t550, t6936);
    (t91348, t91354, t91357, t91359, t91362, t91365, t91366, t91370, t91374)
}
