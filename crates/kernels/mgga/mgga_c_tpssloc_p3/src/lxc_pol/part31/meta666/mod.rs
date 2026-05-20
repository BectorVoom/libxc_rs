//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta666 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1955;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1956;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1957;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1958;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta666<F: Float>(t1081: F, t5527: F, t16596: F, t89992: F, t23788: F, t98007: F, t17109: F, t28: F, t25365: F, t98058: F, t25927: F, t98003: F, t1395: F, t5456: F, t2105: F, t6470: F, t1851: F, t7961: F, t1404: F, t1858: F, t20149: F, t20186: F, t2099: F, t27241: F, t29396: F, t5364: F, t5381: F, t6483: F, t7223: F, t7946: F, t91830: F, t91832: F, t91834: F, t91842: F, t109: F, t84036: F, t86583: F, t86586: F, t92122: F, t92123: F, t96713: F, t96716: F, t96719: F, t96721: F, t96724: F, t96726: F, t2098: F, t671: F, t112: F, t29395: F, t12524: F, t1401: F, t1458: F, t16524: F, t19534: F, t20176: F, t24462: F, t24465: F, t27170: F, t27273: F, t27276: F, t28951: F, t29422: F, t29425: F, t33185: F, t3938: F, t5371: F, t5376: F, t5493: F, t55388: F, t7230: F, t7235: F, t75795: F, t7956: F, t94127: F, t94170: F, t19289: F, t19451: F, t1983: F, t2039: F, t2095: F, t2314: F, t24987: F, t24995: F, t26114: F, t26161: F, t26179: F, t26558: F, t26875: F, t27150: F, t27171: F, t27219: F, t27226: F, t29197: F, t29211: F, t35259: F, t4028: F, t4034: F, t4072: F, t5308: F, t57806: F, t6468: F, t652: F, t7057: F, t7166: F, t7458: F, t7802: F, t7890: F, t7941: F, t96830: F, t97890: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t100759, t100766, t100769, t100772, t100780, t100788, t100791) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1955::<F>(t1081, t5527, t16596, t89992, t23788, t98007, t17109, t28, t25365, t98058, t25927, t98003);
        let (t100930, t100976) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1956::<F>(t1395, t5456, t2105, t6470, t1851, t7961, t1404, t1858, t20149, t20186, t2099, t27241, t29396, t5364, t5381, t6483, t7223, t7946, t91830, t91832, t91834, t91842);
        let (t100990, t100993) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1957::<F>(t109, t84036, t86583, t86586, t92122, t92123, t96713, t96716, t96719, t96721, t96724, t96726, t2098, t671);
        let t101021 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1958::<F>(t112, t29395, t100990, t100993, t12524, t1401, t1458, t16524, t19534, t20176, t24462, t24465, t27170, t27273, t27276, t28951, t29422, t29425, t33185, t3938, t5371, t5376, t5456, t5493, t55388, t671, t7230, t7235, t75795, t7956, t94127, t94170);
        let t101091 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1959::<F>(t19289, t19451, t1983, t2039, t2095, t2314, t24987, t24995, t26114, t26161, t26179, t26558, t26875, t27150, t27171, t27219, t27226, t29197, t29211, t35259, t4028, t4034, t4072, t5308, t57806, t6468, t652, t671, t7057, t7166, t7458, t7802, t7890, t7941, t96830, t97890);
    (t100759, t100766, t100769, t100772, t100780, t100788, t100791, t100930, t100976, t100990, t101021, t101091)
}
