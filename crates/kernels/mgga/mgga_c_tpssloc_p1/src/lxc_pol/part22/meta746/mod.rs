//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta746 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2481;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2482;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2483;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2484;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2485;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2486;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta746<F: Float>(t18030: F, t4630: F, t17884: F, t4644: F, t13969: F, t21502: F, t3039: F, t10214: F, t1041: F, t14080: F, t14164: F, t21603: F, t2979: F, t3048: F, t4582: F, t47775: F, t5861: F, t62282: F, t62284: F, t68521: F, t68534: F, t68539: F, t70330: F, t70339: F, t973: F, t977: F, t1023: F, t14218: F, t14508: F, t17673: F, t17701: F, t17734: F, t21138: F, t21597: F, t3070: F, t3071: F, t3114: F, t42388: F, t42752: F, t4650: F, t48570: F, t48611: F, t49853: F, t49872: F, t49934: F, t5681: F, t62306: F, t69935: F, t21550: F, t10937: F, t17697: F, t21570: F, t2986: F, t42358: F, t43361: F, t49907: F, t49923: F, t50366: F, t62343: F, t62349: F, t62360: F, t62840: F, t68513: F, t70273: F, t135: F, t21537: F, t21541: F, t21545: F, t13995: F, t18041: F, t10390: F, t48496: F, t49984: F, t5909: F, t62418: F, t68458: F, t68466: F, t68470: F, t68543: F, t68547: F, t68554: F, t17659: F, t10422: F, t21573: F, t10408: F, t21516: F, t21520: F, t21574: F, t3117: F, t4337: F, t49994: F, t50048: F, t5857: F, t62441: F, t62445: F, t70442: F, t25548: F, t360: F, t10403: F, t17177: F, t17182: F, t17920: F, t17925: F, t17972: F, t3130: F, t4594: F, t62494: F, t62499: F, t62510: F, t62515: F, t70082: F, t70391: F, t1036: F, t21483: F, t21511: F, t10413: F, t21531: F, t14511: F, t17718: F, t18021: F, t21396: F, t21595: F, t48607: F, t50148: F, t50170: F, t62602: F, t69657: F, t884: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t70554, t70599) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2481::<F>(t18030, t4630, t17884, t4644, t13969, t21502, t3039, t10214, t1041, t14080, t14164, t21603, t2979, t3048, t4582, t47775, t5861, t62282, t62284, t68521, t68534, t68539, t70330, t70339, t973, t977);
        let t70623 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2482::<F>(t1023, t14218, t14508, t17673, t17701, t17734, t21138, t21597, t3070, t3071, t3114, t42388, t42752, t4650, t48570, t48611, t49853, t49872, t49934, t5681, t62306, t69935);
        let t70645 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2483::<F>(t1041, t13969, t21550, t1023, t10937, t14218, t17697, t21570, t2986, t42358, t43361, t4582, t4644, t48611, t49907, t49923, t50366, t62343, t62349, t62360, t62840, t68513, t70273);
        let (t70655, t70660, t70665, t70707) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2484::<F>(t135, t21537, t973, t21541, t21545, t13995, t18041, t10390, t1041, t21570, t2979, t4582, t48496, t49984, t5909, t62418, t68458, t68466, t68470, t68543, t68547, t68554, t70330, t977);
        let t70728 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2485::<F>(t17659, t4644, t10422, t21573, t3070, t10390, t10408, t10937, t14080, t21516, t21520, t21574, t3117, t4337, t49994, t50048, t5857, t62441, t62445, t70442);
        let t70756 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2486::<F>(t25548, t360, t10403, t10408, t13995, t17177, t17182, t17920, t17925, t17972, t3070, t3071, t3130, t4582, t4594, t4644, t49934, t5681, t62494, t62499, t62510, t62515, t70082, t70391);
        let (t70766, t70802) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2487::<F>(t1036, t21483, t1041, t13969, t21511, t10413, t10422, t21531, t10408, t10937, t13995, t14511, t17718, t18021, t21396, t21520, t21595, t3070, t3071, t43361, t48607, t50148, t50170, t62602, t69657, t884);
    (t70554, t70599, t70623, t70645, t70655, t70660, t70665, t70707, t70728, t70756, t70766, t70802)
}
