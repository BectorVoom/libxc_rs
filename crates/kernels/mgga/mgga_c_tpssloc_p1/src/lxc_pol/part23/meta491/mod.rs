//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1502;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1503;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1504;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1505;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1506;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1507;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta491<F: Float>(t80019: F, t80047: F, t6414: F, t550: F, t3792: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t79834: F, t79835: F, t79836: F, t79837: F, t79853: F, t79854: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t79856: F, t79857: F, t79858: F, t79890: F, t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t79896: F, t79897: F, t79898: F, t79899: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t79904: F, t79905: F, t79906: F, t79907: F, t79908: F, t79909: F, t79910: F, t39490: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t79914: F, t39563: F, t39570: F, t39582: F, t39585: F, t39590: F, t39593: F, t39595: F, t79925: F, t79927: F, t79928: F, t79929: F, t79930: F, t79934: F, t39597: F, t39604: F, t39606: F, t39608: F, t39615: F, t39635: F, t79935: F, t79942: F, t79946: F, t79952: F, t79953: F, t79954: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t80048, t80076, t80085, t80101) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1502::<F>(t80019, t80047, t6414, t550, t3792, t39249, t39256, t39261, t39266, t39304, t39309, t79834, t79835, t79836, t79837, t79853, t79854);
        let t80102 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1503::<F>(t39312, t39316, t39320, t39324, t39327, t39338, t39346, t39349, t39356, t79856, t79857, t79858, t79890);
        let t80104 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1504::<F>(t39360, t39364, t39373, t39384, t39393, t39397, t39400, t39408, t79896, t79897, t79898, t79899);
        let t80105 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1505::<F>(t39411, t39463, t39468, t39472, t39476, t39483, t79904, t79905, t79906, t79907, t79908, t79909, t79910);
        let t80108 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1506::<F>(t39490, t39496, t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539, t39549, t79914);
        let t80109 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1507::<F>(t39563, t39570, t39582, t39585, t39590, t39593, t39595, t79925, t79927, t79928, t79929, t79930, t79934);
        let t80111 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1508::<F>(t39597, t39604, t39606, t39608, t39615, t39635, t79935, t79942, t79946, t79952, t79953, t79954);
    (t80048, t80076, t80085, t80101, t80102, t80104, t80105, t80108, t80109, t80111)
}
