//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta739 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2434;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2435;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta739<F: Float>(t17513: F, t49489: F, t10661: F, t21253: F, t912: F, t2842: F, t4395: F, t5695: F, t10702: F, t21268: F, t10817: F, t21315: F, t2792: F, t4396: F, t5726: F, t1557: F, t17422: F, t10655: F, t21318: F, t1556: F, t60745: F, t17520: F, t10771: F, t14271: F, t14276: F, t17535: F, t17538: F, t17541: F, t21259: F, t2886: F, t4433: F, t49430: F, t5743: F, t931: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t69288, t69291, t69294, t69297, t69299) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2434::<F>(t17513, t49489, t10661, t21253, t912, t2842, t4395, t5695, t10702, t21268, t10817, t21315);
        let (t69302, t69305, t69307, t69310, t69313) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2435::<F>(t2792, t4396, t5726, t1557, t17422, t10655, t21318, t1556, t2842, t60745, t17520, t4395);
        let t69326 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2436::<F>(t10771, t14271, t14276, t17535, t17538, t17541, t21259, t2886, t4433, t49430, t5743, t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69313, t931);
    (t69288, t69291, t69294, t69297, t69299, t69302, t69305, t69307, t69310, t69313, t69326)
}
