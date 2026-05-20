//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta570 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2283;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta570<F: Float>(t18188: F, t19288: F, t12560: F, t12561: F, t12562: F, t12563: F, t12564: F, t12565: F, t9225: F, t5385: F, t604: F, t5389: F, t645: F, t1437: F, t4021: F, t5445: F, t1409: F, t65: F, t67: F, t1864: F, t3966: F, t5392: F, t628: F, t17635: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19289, t19297, t19299, t19310) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2283::<F>(t18188, t19288, t12560, t12561, t12562, t12563, t12564, t12565, t9225, t5385, t604, t5389, t645);
        let (t19313, t19318, t19322, t19323, t19326, t19331) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2284::<F>(t1437, t4021, t5445, t645, t1409, t65, t67, t1864, t3966, t5392, t628, t17635);
    (t19289, t19297, t19299, t19310, t19313, t19318, t19322, t19323, t19326, t19331)
}
