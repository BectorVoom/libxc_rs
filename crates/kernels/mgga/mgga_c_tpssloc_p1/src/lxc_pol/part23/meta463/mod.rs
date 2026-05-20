//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1354;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1355;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1356;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta463<F: Float>(t136: F, t2826: F, t76608: F, t76612: F, t908: F, t76616: F, t76620: F, t43002: F, t48103: F, t60168: F, t60173: F, t60204: F, t68452: F, t68454: F, t76903: F, t13769: F, t17794: F, t17804: F, t2986: F, t340: F, t343: F, t4510: F, t4531: F, t61310: F, t61313: F, t69548: F, t69647: F, t69683: F, t69686: F, t69691: F, t69699: F, t69727: F, t69739: F, t69746: F, t76593: F, t76901: F, t973: F, t974: F, t10214: F, t10217: F, t10278: F, t1597: F, t21444: F, t2979: F, t2980: F, t42976: F, t4546: F, t48336: F, t48397: F, t61408: F, t61489: F, t61597: F, t61600: F, t69796: F, t69801: F, t69806: F, t75836: F, t75847: F, t977: F, t76829: F, t76865: F, t225: F, t76634: F, t76636: F, t76641: F, t76643: F, t76647: F, t76652: F, t76654: F, t76657: F, t76659: F, t76661: F, t76663: F) -> (F, F, F, F, F, F, F) {
        let (t76906, t76909, t76912, t76915, t76922) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1354::<F>(t136, t2826, t76608, t76612, t908, t76616, t76620, t43002, t48103, t60168, t60173, t60204, t68452, t68454, t76903);
        let t76943 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1355::<F>(t13769, t17794, t17804, t2986, t340, t343, t4510, t4531, t61310, t61313, t69548, t69647, t69683, t69686, t69691, t69699, t69727, t69739, t69746, t76593, t76901, t76922, t973, t974);
        let t76974 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1356::<F>(t10214, t10217, t10278, t1597, t21444, t2979, t2980, t343, t42976, t4546, t48336, t48397, t61408, t61489, t61597, t61600, t69796, t69801, t69806, t75836, t75847, t973, t977);
        let (t76976, t76977, t76995) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1357::<F>(t76829, t76865, t76943, t76974, t225, t76634, t76636, t76641, t76643, t76647, t76652, t76654, t76657, t76659, t76661, t76663);
    (t76906, t76909, t76912, t76915, t76976, t76977, t76995)
}
