//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1241/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1241<F: Float>(t225: F, t26221: F, t26329: F, t26229: F, t111: F, t27370: F, t112: F, t27907: F, t8110: F, t2022: F, t671: F, t7450: F) -> (F, F, F, F, F, F, F, F) {
    let t91441 = t26221 * t225;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t96238 = t27370 * t111;
    let t96311 = t27907 * t112;
    let t96334 = t8110 * t111;
    let t96351 = t2022 * t671;
    let t96361 = t7450 * t671;
    (t91441, t91488, t91491, t96238, t96311, t96334, t96351, t96361)
}
