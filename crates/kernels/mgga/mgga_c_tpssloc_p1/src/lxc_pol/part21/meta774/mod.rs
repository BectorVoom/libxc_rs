//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta774 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2680;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2681;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta774<F: Float>(t25: F, t54408: F, t54411: F, t12061: F, t15937: F, t16557: F, t19547: F, t19552: F, t21: F, t2249: F, t3664: F, t3665: F, t39419: F, t5134: F, t5397: F, t54347: F, t56226: F, t584: F, t606: F, t6305: F, t9: F, t9212: F, zeta_threshold: F, t28: F, t1081: F, t12072: F, t15952: F, t18196: F, t19559: F, t19564: F, t3231: F, t3672: F, t3673: F, t39436: F, t5142: F, t54370: F, t56252: F, t5966: F, t6312: F, t157: F, t182: F, t1390: F, t20063: F, t54412: F, t39491: F, t12466: F, t1307: F, t16148: F, t3918: F, t39483: F, t39490: F, t39496: F, t5122: F, t5126: F, t6330: F) -> (F, F, F, F, F, F, F) {
        let (t56298, t56299, t56323) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2680::<F>(t25, t54408, t54411, t12061, t15937, t16557, t19547, t19552, t21, t2249, t3664, t3665, t39419, t5134, t5397, t54347, t56226, t584, t606, t6305, t9, t9212, zeta_threshold);
        let t56347 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2681::<F>(t28, t1081, t12072, t15952, t18196, t19559, t19564, t21, t3231, t3672, t3673, t39436, t5142, t54370, t56252, t584, t5966, t6312, t9, t9212, zeta_threshold);
        let (t56349, t56351, t56362, t56363, t56364) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2682::<F>(t157, t56323, t56347, t182, t1390, t20063, t54412, t39491, t12466, t1307, t16148, t3918, t39483, t39490, t39496, t5122, t5126, t56298, t56299, t6330);
    (t56298, t56299, t56349, t56351, t56362, t56363, t56364)
}
