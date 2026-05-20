//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta229 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta229<F: Float>(t111: F, t1851: F, t5392: F, t9427: F, t9438: F, t5520: F, t751: F, t2658: F, t5660: F, t870: F, t172: F, t5522: F) -> (F, F, F, F, F, F, F, F) {
        let (t16524, t16549, t16563, t16578, t16586, t16587, t16606, t16616) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk879::<F>(t111, t1851, t5392, t9427, t9438, t5520, t751, t2658, t5660, t870, t172, t5522);
    (t16524, t16549, t16563, t16578, t16586, t16587, t16606, t16616)
}
