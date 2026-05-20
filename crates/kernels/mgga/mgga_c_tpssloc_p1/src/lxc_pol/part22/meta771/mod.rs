//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta771 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2625;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2626;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2627;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2628;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2629;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2630;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta771<F: Float>(t1174: F, t22059: F, t3431: F, t50846: F, t63841: F, t63843: F, t63845: F, t63886: F, t63888: F, t63893: F, t63911: F, t71333: F, t71335: F, t71337: F, t71400: F, t71406: F, t71408: F, t71411: F, t71414: F, t71417: F, t71420: F, t71423: F, t71426: F, t43859: F, t44466: F, t52313: F, t52339: F, t52343: F, t64074: F, t64076: F, t64087: F, t64089: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t71486: F, t71489: F, t71505: F, t71508: F, t71511: F, t18529: F, t4889: F, t135: F, t22034: F, t15338: F, t18409: F, t3447: F, t15320: F, t15376: F, t18427: F, t18434: F, t52058: F, t64711: F, t64713: F, t64718: F, t64730: F, t64733: F, t20217: F, t3450: F, t18469: F, t52059: F, t4904: F, t64763: F, t18532: F, t22040: F, t18321: F, t4916: F, t1187: F, t18437: F, t18526: F, t3449: F, t4908: F, t4931: F, t52074: F, t52081: F, t52085: F, t64765: F, t64770: F, t64773: F, t64781: F, t64784: F, t64821: F, t71177: F, t73113: F, t11583: F, t21510: F, t11570: F, t15382: F, t18484: F, t44478: F, t4919: F, t5979: F, t64648: F, t64951: F, t64969: F, t64976: F, t64979: F, t64981: F, t64988: F, t65077: F, t7319: F, t15419: F, t21745: F, t20234: F, t44505: F, t1171: F, t22104: F, t15313: F, t18416: F, t4920: F, t64756: F, t64775: F, t64811: F, t65035: F, t65041: F, t65093: F, t65112: F, t65126: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73330, t73355) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2625::<F>(t1174, t22059, t3431, t50846, t63841, t63843, t63845, t63886, t63888, t63893, t63911, t71333, t71335, t71337, t71400, t71406, t71408, t71411, t71414, t71417, t71420, t71423, t71426);
        let t73369 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2626::<F>(t43859, t44466, t52313, t52339, t52343, t64074, t64076, t64087, t64089, t71470, t71472, t71474, t71477, t71480, t71483, t71486, t71489, t71505, t71508, t71511);
        let t73399 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2627::<F>(t18529, t4889, t1174, t135, t22034, t15338, t18409, t3447, t15320, t15376, t18427, t18434, t52058, t64711, t64713, t64718, t64730, t64733);
        let (t73405, t73417, t73420, t73424, t73427) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2628::<F>(t20217, t3450, t18469, t3447, t52059, t4904, t64763, t18532, t4889, t1174, t135, t22040);
        let t73439 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2629::<F>(t18321, t4916, t1187, t15376, t18437, t18526, t3447, t3449, t4889, t4908, t4931, t52074, t52081, t52085, t64765, t64770, t64773, t64781, t64784, t64821, t71177, t73113, t73405, t73417, t73420, t73424, t73427);
        let (t73444, t73451, t73480) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2630::<F>(t11583, t21510, t11570, t15376, t15382, t18484, t3447, t44478, t4919, t5979, t64648, t64951, t64969, t64976, t64979, t64981, t64988, t65077, t7319);
        let (t73491, t73496, t73525) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2631::<F>(t15419, t21745, t3447, t20234, t44505, t1171, t22104, t15313, t15320, t18409, t18416, t4904, t4919, t4920, t64756, t64775, t64811, t65035, t65041, t65093, t65112, t65126);
    (t73330, t73355, t73369, t73399, t73439, t73444, t73451, t73480, t73491, t73496, t73525)
}
