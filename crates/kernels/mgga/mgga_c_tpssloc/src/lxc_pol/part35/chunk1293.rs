//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1293/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1293<F: Float>(t1089: F, t3597: F, t24588: F, t8020: F, t2121: F, t3427: F, t8010: F, t1751: F, t225: F, t461: F, t8006: F, t85660: F) -> (F, F, F, F, F) {
    let t94332 = t3597 * t1089;
    let t94395 = t8020 * t24588;
    let t94436 = t2121 * t3427 * t8010;
    let t94458 = t461 * t1751 * t225;
    let t94476 = t85660 * t8006;
    (t94332, t94395, t94436, t94458, t94476)
}
