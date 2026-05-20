//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta773 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2675;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2676;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2677;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2678;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2679;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta773<F: Float>(t1388: F, t5187: F, t1307: F, t5356: F, t54392: F, t54395: F, t54398: F, t54400: F, t15904: F, t20077: F, t20085: F, t3734: F, t3918: F, t39463: F, t39468: F, t39472: F, t5126: F, t5161: F, t25: F, t54402: F, t2: F, t584: F, t606: F, t11987: F, t15989: F, t16557: F, t19606: F, t19611: F, t21: F, t2249: F, t3665: F, t3704: F, t39861: F, t5170: F, t53825: F, t5397: F, t6305: F, t9: F, t9212: F, zeta_threshold: F, t28: F, t1081: F, t12000: F, t16003: F, t18196: F, t19618: F, t19623: F, t3231: F, t3673: F, t3711: F, t39877: F, t5178: F, t53852: F, t5966: F, t6312: F, t54405: F, t12466: F, t1297: F, t15868: F, t1799: F, t193: F, t19577: F, t19596: F, t19994: F, t3719: F, t3914: F, t3919: F, t39476: F, t5160: F, t55191: F, t55266: F, t6301: F, t6347: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t56202, t56203, t56207, t56208, t56212) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2675::<F>(t1388, t5187, t1307, t5356, t54392, t54395, t54398, t54400, t15904, t20077, t20085, t3734, t3918, t39463, t39468, t39472, t5126, t5161);
        let (t56219, t56226, t56247) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2676::<F>(t25, t54402, t2, t584, t606, t11987, t15989, t16557, t19606, t19611, t21, t2249, t3665, t3704, t39861, t5170, t53825, t5397, t6305, t9, t9212, zeta_threshold);
        let (t56252, t56273) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2677::<F>(t28, t1081, t2, t584, t12000, t16003, t18196, t19618, t19623, t21, t3231, t3673, t3711, t39877, t5178, t53852, t5966, t6312, t9, t9212, zeta_threshold);
        let t56275 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2678::<F>(t56247, t56273);
        let (t56279, t56294) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2679::<F>(t54405, t12466, t1297, t15868, t15904, t1799, t193, t19577, t19596, t19994, t20077, t3719, t3914, t3918, t3919, t39476, t5126, t5160, t55191, t55266, t56219, t56275, t6301, t6347);
    (t56202, t56203, t56207, t56208, t56212, t56219, t56226, t56252, t56275, t56279, t56294)
}
