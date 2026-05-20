//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk872;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk873;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk874;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk875;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk876;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk877;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta164<F: Float>(t3566: F, t484: F, t121: F, t486: F, t1216: F, t248: F, t1213: F, t478: F, t483: F, t3068: F, t1244: F, t1230: F, t820: F, t1090: F, t1089: F, t415: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3567, t3570) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk872::<F>(t3566, t484, t121, t486);
        let t3572 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk873::<F>(t1216, t248, t3570);
        let (t3573, t3575, t3576, t3577) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk874::<F>(t1213, t3572, t478, t483, t3068, t1244);
        let t3578 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk875::<F>(t1230, t820);
        let (t3579, t3580) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk876::<F>(t1090, t1216, t3578);
        let t3584 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk877::<F>(t1089, t415);
    (t3567, t3570, t3572, t3573, t3575, t3576, t3577, t3578, t3579, t3580, t3584)
}
