//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1098/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1098<F: Float>(t107: F, t240: F, t625: F, t656: F, t2331: F, t63: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F, F) {
    let t22468 = t240 * t107;
    let t22469 = F::new(11.0) / F::new(9.0) * t22468;
    let t22470 = t625 * t656;
    let t22473 = t63 * t2331;
    let t22510 = F::new(88.0) / F::new(9.0) * t240;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    (t22469, t22470, t22473, t22510, t22573, t22574)
}
