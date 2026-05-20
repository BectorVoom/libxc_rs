//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta835 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2962;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2963;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2964;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2965;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2967;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2968;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta835<F: Float>(t4649: F, t1009: F, t17875: F, t1011: F, t1019: F, t3030: F, t5848: F, t3032: F, t3129: F, t3038: F, t10891: F, t17655: F, t17884: F, t3117: F, t18029: F, t3108: F, t1021: F, t1025: F, t10863: F, t10957: F, t10965: F, t1618: F, t17607: F, t248: F, t3043: F, t3057: F, t3064: F, t3098: F, t3130: F, t3131: F, t3134: F, t48446: F, t49678: F, t5857: F, t5861: F, t5900: F, t17919: F, t3070: F, t42488: F, t1022: F, t3966: F, t360: F, t1041: F, t10868: F, t5685: F, t14134: F, t4644: F, t13961: F, t4641: F, t14137: F, t12606: F, t1409: F, t10408: F, t13555: F, t13559: F, t14077: F, t1616: F, t17632: F, t17962: F, t3071: F, t3109: F, t42743: F, t4337: F, t4582: F, t4583: F, t4652: F, t48460: F, t48463: F, t5880: F, t10413: F, t13977: F, t13982: F, t13987: F, t13991: F, t14099: F, t14103: F, t14508: F, t14511: F, t17673: F, t17693: F, t3041: F, t3048: F, t42432: F, t42561: F, t4347: F, t4650: F, t48548: F, t48564: F, t48567: F, t48570: F, t48574: F, t50265: F, t5677: F, t13969: F, t17971: F, t2244: F, t5392: F, t17713: F, t884: F, t1023: F, t10390: F, t10403: F, t14211: F, t17187: F, t17688: F, t17972: F, t18021: F, t3121: F, t3132: F, t4579: F, t47775: F, t48626: F, t48629: F, t48670: F, t48674: F, t50324: F, t2250: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61719, t61729, t61731, t61734, t61736, t61739, t61742) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2962::<F>(t4649, t1009, t17875, t1011, t1019, t3030, t5848, t3032, t3129, t3038, t10891, t17655);
        let t61760 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2963::<F>(t17884, t3117, t18029, t3108, t1021, t1025, t10863, t10957, t10965, t1618, t17607, t248, t3043, t3057, t3064, t3098, t3130, t3131, t3134, t48446, t49678, t5857, t5861, t5900, t61719, t61731, t61736, t61739, t61742);
        let (t61768, t61775, t61782, t61784) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2964::<F>(t17919, t3070, t42488, t1022, t3966, t360, t1041, t10868, t248, t5685, t14134, t4644);
        let (t61798, t61803) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2965::<F>(t13961, t4641, t14137, t4644, t12606, t1409, t10408, t1041, t10891, t13555, t13559, t14077, t1616, t17632, t17962, t3070, t3071, t3109, t42743, t4337, t4582, t4583, t4652, t48460, t48463, t5880, t61768, t61775, t61782, t61784);
        let t61835 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2966::<F>(t10408, t10413, t13977, t13982, t13987, t13991, t14099, t14103, t14508, t14511, t17673, t17693, t3041, t3048, t3070, t3071, t42432, t42561, t4347, t4650, t48548, t48564, t48567, t48570, t48574, t50265, t5677);
        let (t61853, t61855) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2967::<F>(t1041, t13969, t17971, t2244, t5392);
        let (t61871, t61876) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2968::<F>(t13969, t17713, t3130, t4649, t884, t1023, t10390, t10403, t10408, t1041, t14211, t17187, t17688, t17972, t18021, t3048, t3070, t3071, t3121, t3132, t4579, t4582, t47775, t48626, t48629, t48670, t48674, t50324, t5677, t61853, t61855);
        let t61910 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2969::<F>(t2250, t5392);
    (t61719, t61729, t61734, t61760, t61775, t61798, t61803, t61835, t61855, t61871, t61876, t61910)
}
