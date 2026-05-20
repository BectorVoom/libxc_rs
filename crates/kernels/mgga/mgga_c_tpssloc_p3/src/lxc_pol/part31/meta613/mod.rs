//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1858;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta613<F: Float>(t1441: F, t4072: F, t19440: F, t71: F, t33: F, t55880: F, t5441: F, t645: F, t72: F, t5389: F, t641: F, t12568: F, t1410: F, t27960: F, t4021: F, t7431: F, t1864: F, t12571: F, t27971: F, t1437: F, t7445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96356, t96379, t96383, t96393, t96403, t96406) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1858::<F>(t1441, t4072, t19440, t71, t33, t55880, t5441, t645, t72, t5389, t641, t12568, t1410);
        let (t96418, t96422, t96425, t96443, t96458, t96461) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1859::<F>(t27960, t645, t72, t4021, t7431, t1864, t5389, t12571, t1410, t27971, t1437, t7445);
    (t96356, t96379, t96383, t96393, t96403, t96406, t96418, t96422, t96425, t96443, t96458, t96461)
}
