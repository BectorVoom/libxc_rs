//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta617<F: Float>(t225: F, t23410: F, t6692: F, t82632: F, t6707: F, t82573: F, t6695: F, t3166: F, t6703: F, t1049: F, t6733: F, t23366: F, t23384: F) -> (F, F, F, F, F, F, F) {
        let (t83276, t83281, t83285, t83287, t83296, t83303, t83316) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2095::<F>(t225, t23410, t6692, t82632, t6707, t82573, t6695, t3166, t6703, t1049, t6733, t23366, t23384);
    (t83276, t83281, t83285, t83287, t83296, t83303, t83316)
}
