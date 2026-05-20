//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta774 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2647;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2648;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta774<F: Float>(t28: F, t1081: F, t5966: F, t584: F, t15952: F, t15955: F, t18196: F, t19559: F, t20385: F, t20390: F, t2219: F, t3672: F, t39436: F, t5142: F, t517: F, t71090: F, zeta_threshold: F, t157: F, t73989: F, t182: F, t20675: F, t3701: F, t39305: F, t1388: F, t20077: F, t20681: F, t3918: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t5160: F, t5187: F, t53783: F, t53788: F, t53797: F, t55224: F, t73958: F, t73959: F, t73960: F, t73961: F, t73962: F, t73968: F, t73969: F, t54312: F, t39328: F, t39339: F, t39341: F, t6347: F, t54325: F, t20416: F, t3919: F, t39338: F, t39346: F, t39349: F, t39356: F, t39360: F, t5161: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t73995, t73998, t74009) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2647::<F>(t28, t1081, t5966, t584, t15952, t15955, t18196, t19559, t20385, t20390, t2219, t3672, t39436, t5142, t517, t71090, zeta_threshold);
        let (t74011, t74013, t74017, t74020) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2648::<F>(t157, t73989, t74009, t182, t20675, t3701, t39305, t1388, t20077, t20681, t3918, t39249, t39256, t39261, t39266, t39304, t5160, t5187, t53783, t53788, t53797, t55224, t73958, t73959, t73960, t73961, t73962, t73968, t73969);
        let (t74024, t74026, t74027, t74028, t74036, t74037) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2649::<F>(t54312, t39328, t39339, t39341, t1388, t6347, t54325, t20416, t3918, t3919, t39338, t39346, t39349, t39356, t39360, t5161);
    (t73995, t73998, t74011, t74013, t74017, t74020, t74024, t74026, t74027, t74028, t74036, t74037)
}
