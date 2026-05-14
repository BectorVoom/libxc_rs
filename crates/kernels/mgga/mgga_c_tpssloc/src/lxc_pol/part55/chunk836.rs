//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 836/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk836<F: Float>(t131: F, t24810: F, t467: F, t3030: F, t461: F, t1011: F, t3508: F, t1193: F, t7372: F, t7378: F, t7319: F, t7327: F, t1170: F, t7381: F, t2121: F, t210: F, t7371: F) -> (F, F, F, F, F, F, F, F) {
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    let t24813 = t461 * t3030;
    let t24815 = t1011 * t3508;
    let t24826 = t7372 * t1193;
    let t24827 = t24826 * t7378;
    let t24833 = t7319 * t7327;
    let t24844 = t1170 * t7381;
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    (t24812, t24813, t24815, t24826, t24827, t24833, t24845, t24847)
}
