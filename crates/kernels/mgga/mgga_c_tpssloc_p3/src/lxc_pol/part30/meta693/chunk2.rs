//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2210/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2210<F: Float>(t2717: F, t5636: F, t22986: F, t23270: F, t776: F, t225: F, t28437: F, t258: F, t5544: F, t25038: F, t1888: F, t5657: F, t865: F) -> (F, F, F, F) {
    let t98161 = t2717 * t5636;
    let t98164 = t22986 * t23270 * t98161 * t776;
    let t98166 = t28437 * t225;
    let t98169 = t258 * t5544;
    let t98172 = t25038 * t23270 * t98169 * t776;
    let t98181 = t1888 * t23270 * t2717 * t5657 * t865;
    (t98164, t98166, t98172, t98181)
}
