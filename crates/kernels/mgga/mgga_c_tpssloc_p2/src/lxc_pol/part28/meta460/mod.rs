//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta460<F: Float>(t1484: F, t258: F, t776: F, t23270: F, t25038: F, t1527: F, t2717: F, t865: F, t1888: F, t6547: F, t7485: F, t857: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t25039, t25040, t25041, t25042, t25044, t25045, t25046, t25047, t25049, t25053) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1668::<F>(t1484, t258, t776, t23270, t25038, t1527, t2717, t865, t1888, t6547, t7485, t857);
    (t25039, t25040, t25041, t25042, t25044, t25045, t25046, t25047, t25049, t25053)
}
