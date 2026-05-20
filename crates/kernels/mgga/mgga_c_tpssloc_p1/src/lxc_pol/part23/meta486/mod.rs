//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1490;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1491;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta486<F: Float>(t25: F, t54312: F, t54314: F, t54316: F, t6305: F, t5397: F, t19547: F, t20216: F, t3664: F, t39419: F, t5134: F, t514: F, t75911: F, zeta_threshold: F, t28: F, t6312: F, t5966: F, t19559: F, t20390: F, t3672: F, t39436: F, t5142: F, t517: F, t77953: F, t157: F, t182: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t79856, t79857, t79858, t79859, t79864, t79872) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1490::<F>(t25, t54312, t54314, t54316, t6305, t5397, t19547, t20216, t3664, t39419, t5134, t514, t75911, zeta_threshold);
        let (t79873, t79878, t79888, t79890) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1491::<F>(t28, t6312, t5966, t19559, t20390, t3672, t39436, t5142, t517, t77953, t157, t79872, t182, zeta_threshold);
        let t79891 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1492::<F>(t39266, t39304, t39309, t39312, t39316, t39320, t39324, t39327, t39338, t39346, t39349, t79856, t79857, t79858, t79890);
    (t79856, t79857, t79858, t79859, t79864, t79873, t79878, t79888, t79890, t79891)
}
