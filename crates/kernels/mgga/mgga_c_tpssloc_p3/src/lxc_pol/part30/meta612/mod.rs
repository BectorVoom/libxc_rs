//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2008;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta612<F: Float>(t23518: F, t6733: F, t23669: F, t995: F, t3158: F, t6796: F, t6802: F, t23600: F, t10336: F, t1920: F, t1949: F, t2966: F, t6805: F, t135: F, t23631: F, t6688: F, t23617: F, t6680: F, t10889: F, t3033: F, t6753: F, t10510: F, t6755: F, t10870: F, t6765: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82683, t82713, t82716, t82717, t82736, t82799, t82809) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2008::<F>(t23518, t6733, t23669, t995, t3158, t6796, t6802, t23600, t10336, t1920, t1949, t2966, t6805);
        let (t82822, t82830, t82848, t82851, t82875) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2009::<F>(t135, t23631, t6688, t23617, t6680, t10889, t3033, t6753, t10510, t6755, t10870, t6765);
    (t82683, t82713, t82716, t82717, t82736, t82799, t82809, t82822, t82830, t82848, t82851, t82875)
}
