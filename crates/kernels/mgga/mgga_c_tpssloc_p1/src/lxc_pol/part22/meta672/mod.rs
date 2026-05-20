//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta672 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2227;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta672<F: Float>(t17152: F, t2986: F, t48213: F, t17863: F, t42837: F, t10186: F, t17808: F, t10236: F, t17635: F, t13835: F, t13847: F, t13839: F, t48279: F, t17748: F, t17849: F, t2960: F, t5838: F, t698: F, t973: F, t5844: F, t4509: F, t5836: F, t10190: F, t17794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61261, t61264, t61273, t61279, t61288, t61291) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2227::<F>(t17152, t2986, t48213, t17863, t42837, t10186, t17808, t10236, t17635, t13835, t13847, t13839, t48279);
        let (t61294, t61307, t61310, t61313, t61322, t61327) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2228::<F>(t13847, t17748, t2986, t17849, t2960, t5838, t698, t973, t5844, t4509, t5836, t10190, t17794);
    (t61261, t61264, t61273, t61279, t61288, t61291, t61294, t61307, t61310, t61313, t61322, t61327)
}
