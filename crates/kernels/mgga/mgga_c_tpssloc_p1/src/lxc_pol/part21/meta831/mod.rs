//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2928;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2929;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2930;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta831<F: Float>(t1068: F, t4696: F, t13508: F, t4483: F, t17934: F, t2948: F, t13718: F, t10723: F, t17954: F, t959: F, t17937: F, t2925: F, t14667: F, t17198: F, t17202: F, t3209: F, t4700: F, t60398: F, t60400: F, t60429: F, t60434: F, t60568: F, t60570: F, t17297: F, t2929: F, t4497: F, t2904: F, t59975: F, t951: F, t18065: F, t225: F, t10165: F, t10170: F, t1052: F, t1066: F, t11010: F, t13939: F, t14658: F, t1625: F, t1634: F, t1635: F, t17583: F, t18062: F, t18166: F, t3026: F, t3169: F, t3174: F, t3175: F, t3206: F, t388: F, t43604: F, t4552: F, t4657: F, t50625: F, t50632: F, t50653: F, t50703: F, t5919: F, t5920: F, t5944: F, t10160: F, t13736: F, t13743: F, t14526: F, t14545: F, t14549: F, t14555: F, t14659: F, t1603: F, t17575: F, t17588: F, t3166: F, t3176: F, t4557: F, t4660: F, t4665: F, t4694: F, t5848: F, t5943: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60941, t60946, t60953, t60955, t60958, t60961) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2928::<F>(t1068, t4696, t13508, t4483, t17934, t2948, t13718, t10723, t17954, t959, t17937, t2925);
        let t60962 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2929::<F>(t14667, t17198, t17202, t3209, t4700, t60398, t60400, t60429, t60434, t60568, t60570, t60941, t60946, t60953, t60955, t60958, t60961);
        let (t60966, t60970, t61010) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2930::<F>(t17297, t2929, t4497, t959, t2904, t59975, t951, t18065, t225, t10165, t10170, t1052, t1066, t11010, t13939, t14658, t1625, t1634, t1635, t17583, t18062, t18166, t3026, t3169, t3174, t3175, t3206, t388, t43604, t4552, t4657, t50625, t50632, t50653, t50703, t5919, t5920, t5944);
        let t61048 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2931::<F>(t10160, t1052, t13736, t13743, t14526, t14545, t14549, t14555, t14659, t1603, t17575, t17583, t17588, t18062, t3166, t3169, t3174, t3176, t3206, t388, t4557, t4660, t4665, t4694, t5848, t5943, t5944);
    (t60946, t60953, t60955, t60958, t60961, t60962, t60966, t60970, t61010, t61048)
}
