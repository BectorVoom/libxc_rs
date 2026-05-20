//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1690;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta417<F: Float>(t16225: F, t3807: F, t16224: F, t12289: F, t242: F, t1336: F, t16048: F, t5248: F, t5249: F, t12283: F, t5259: F, t5293: F, t120: F, t5286: F, t3805: F, t12407: F, t12284: F, t12301: F, t12397: F, t12429: F, t1341: F, t1363: F, t16147: F, t16150: F, t16155: F, t16159: F, t16208: F, t16211: F, t16214: F, t16217: F, t1827: F, t3778: F, t3803: F, t5289: F) -> (F, F, F, F, F, F, F) {
        let (t16226, t16227, t16233, t16235, t16239, t16241) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1690::<F>(t16225, t3807, t16224, t12289, t242, t1336, t16048, t5248, t5249, t12283, t5259, t5293);
        let (t16242, t16244, t16248, t16253) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1691::<F>(t120, t5286, t3805, t3807, t12407, t5249, t12284, t12301, t12397, t12429, t1341, t1363, t16147, t16150, t16155, t16159, t16208, t16211, t16214, t16217, t16227, t16233, t16235, t16239, t16241, t1827, t3778, t3803, t5259, t5289);
    (t16226, t16227, t16235, t16242, t16244, t16248, t16253)
}
