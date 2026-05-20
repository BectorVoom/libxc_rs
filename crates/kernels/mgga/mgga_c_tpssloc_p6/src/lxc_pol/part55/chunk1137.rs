//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1137/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1137<F: Float>(t2006: F, t5286: F, t225: F, t26221: F, t26329: F, t26229: F, t24600: F, t7301: F, t24615: F, t24588: F, t8020: F, t1751: F, t461: F) -> (F, F, F, F, F, F, F, F) {
    let t90946 = t2006 * t5286;
    let t91441 = t26221 * t225;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t94369 = t24600 * t7301;
    let t94378 = t24600 * t24615;
    let t94395 = t8020 * t24588;
    let t94458 = t461 * t1751 * t225;
    (t90946, t91441, t91488, t91491, t94369, t94378, t94395, t94458)
}
