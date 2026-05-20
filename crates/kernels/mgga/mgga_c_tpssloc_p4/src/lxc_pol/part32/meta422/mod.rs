//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1627;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta422<F: Float>(t11883: F, t1215: F, t6252: F, t1751: F, t5011: F, t1246: F, t6238: F, t19145: F, t3612: F, t1734: F, t5052: F, t1235: F, t6218: F, t19120: F, t493: F, t1243: F, t19045: F, t1755: F, t11881: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1758: F, t18572: F, t3604: F, t3610: F, t470: F, t494: F, t4964: F, t5064: F, t5073: F, t5076: F, t5086: F, t6168: F, t6257: F, t6265: F) -> (F, F, F, F, F, F, F, F) {
        let (t19165, t19166, t19169, t19170, t19173, t19174, t19176, t19179, t19180, t19189) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1627::<F>(t11883, t1215, t6252, t1751, t5011, t1246, t6238, t19145, t3612, t1734, t5052, t1235, t6218);
        let (t19201, t19203, t19207) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1628::<F>(t1246, t19189, t19120, t493, t1243, t19045, t3612, t5011, t1755, t11881, t1201, t1244, t1247, t1249, t1729, t1758, t18572, t19166, t19170, t19174, t19176, t19180, t3604, t3610, t470, t494, t4964, t5064, t5073, t5076, t5086, t6168, t6257, t6265);
    (t19165, t19169, t19173, t19179, t19189, t19201, t19203, t19207)
}
