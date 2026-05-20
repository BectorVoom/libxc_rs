//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta651<F: Float>(t1598: F, t974: F, t23631: F, t1920: F, t25535: F, t968: F, t23665: F, t25479: F, t25487: F, t82736: F, t25493: F, t7611: F, t82713: F) -> (F, F, F, F, F, F, F) {
        let (t89242, t89243, t89256, t89292, t89294, t89296, t89309) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2065::<F>(t1598, t974, t23631, t1920, t25535, t968, t23665, t25479, t25487, t82736, t25493, t7611, t82713);
    (t89242, t89243, t89256, t89292, t89294, t89296, t89309)
}
