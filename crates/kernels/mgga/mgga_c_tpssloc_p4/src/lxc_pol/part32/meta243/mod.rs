//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1102;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta243<F: Float>(t3: F, t6470: F, t1401: F, t1458: F, t3941: F, t5371: F, t5456: F, t5493: F, t577: F, t2235: F, t33: F, t645: F, t79: F, t72: F, t605: F, t608: F, t625: F, t641: F, t71: F, t1874: F, t2314: F, t4034: F, t1266: F, t1873: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6471, t6483, t6486) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1102::<F>(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t2235, t33);
        let (t6492, t6495, t6503, t6509, t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1103::<F>(t645, t79, t72, t605, t608, t625, t641, t71, t1874, t2314, t4034, t1266, t1873);
    (t6471, t6483, t6486, t6492, t6495, t6503, t6509, t6522, t6524, t6525)
}
