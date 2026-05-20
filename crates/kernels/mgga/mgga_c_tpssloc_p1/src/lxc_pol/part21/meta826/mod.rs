//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta826 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2912;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2913;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2914;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2915;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2916;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta826<F: Float>(t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F, t10760: F, t10828: F, t14266: F, t14329: F, t1569: F, t17350: F, t17428: F, t17499: F, t2856: F, t2881: F, t2889: F, t2906: F, t2907: F, t2924: F, t2930: F, t2932: F, t41816: F, t41826: F, t41981: F, t4411: F, t4434: F, t48771: F, t48779: F, t48890: F, t5743: F, t5759: F, t5794: F, t59975: F, t60407: F, t60424: F, t60429: F, t60434: F, t60568: F, t60570: F, t60585: F, t60601: F, t60618: F, t60634: F, t60649: F, t60665: F, t60682: F, t924: F, t932: F, t950: F, t10632: F, t5790: F, t10655: F, t17521: F, t17423: F, t2792: F, t912: F, t17422: F, t2844: F, t2842: F, t17524: F, t17528: F, t42023: F, t10756: F, t10765: F, t13716: F, t14271: F, t14276: F, t14425: F, t14429: F, t14432: F, t14436: F, t17492: F, t17535: F, t2905: F, t42111: F, t42113: F, t4416: F, t4438: F, t4475: F, t48789: F, t49427: F, t49430: F, t5774: F, t5791: F, t17349: F, t2888: F, t13520: F, t14422: F, t10740: F, t10747: F, t10825: F, t14263: F, t14337: F, t14450: F, t14460: F, t17443: F, t17446: F, t17451: F, t17454: F, t17493: F, t17538: F, t17541: F, t17544: F, t17548: F, t17551: F, t17555: F, t2861: F, t2886: F, t41984: F, t42128: F, t42149: F, t4454: F, t4476: F, t49096: F, t49411: F, t60360: F, t931: F, t300: F, t59928: F, t59982: F, t60030: F, t60346: F, t60401: F, t17955: F, t2940: F, t17930: F) -> (F, F, F, F, F, F, F, F, F) {
        let t60698 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2912::<F>(t59748, t59753, t59757, t59759, t59761, t59765, t59769, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let t60711 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2913::<F>(t10760, t10828, t14266, t14329, t1569, t17350, t17428, t17499, t2856, t2881, t2889, t2906, t2907, t2924, t2930, t2932, t41816, t41826, t41981, t4411, t4434, t48771, t48779, t48890, t5743, t5759, t5794, t59975, t60407, t60424, t60429, t60434, t60568, t60570, t60585, t60601, t60618, t60634, t60649, t60665, t60682, t60698, t924, t932, t950);
        let (t60722, t60741, t60744, t60748, t60750, t60752) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2914::<F>(t10632, t5790, t10655, t17521, t17423, t2792, t912, t17422, t2844, t2842, t17524, t17528, t42023);
        let t60763 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2915::<F>(t10756, t10765, t10828, t13716, t14271, t14276, t14425, t14429, t14432, t14436, t17492, t17499, t17535, t2905, t2906, t2924, t2930, t42111, t42113, t4416, t4438, t4475, t48789, t49427, t49430, t5774, t5791, t60722, t60741, t60744, t60748, t60750, t60752);
        let (t60787, t60806) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2916::<F>(t17349, t2888, t13520, t14422, t10740, t10747, t10765, t10825, t14263, t14337, t14450, t14460, t17350, t17443, t17446, t17451, t17454, t17493, t17538, t17541, t17544, t17548, t17551, t17555, t2861, t2886, t41984, t42128, t42149, t4454, t4476, t49096, t49411, t60360, t931, t932);
        let (t60810, t60812, t60814) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2917::<F>(t300, t59928, t59982, t60030, t60346, t60401, t60711, t60763, t60806, t17955, t2940, t17930);
    (t60741, t60744, t60748, t60750, t60752, t60787, t60810, t60812, t60814)
}
