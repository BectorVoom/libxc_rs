//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 744/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk744<F: Float>(t1043: F, t2775: F, t3961: F, t4582: F, t2770: F, t3061: F, t1615: F, t376: F) -> (F, F, F, F, F, F, F) {
    let t4583 = t1043 * t2775;
    let t4584 = t4583 * t3961;
    let t4585 = t4582 * t4584;
    let t4588 = t3061 * t2770;
    let t4589 = t4588 * t3961;
    let t4590 = t4582 * t4589;
    let t4593 = t376 * t1615;
    (t4583, t4584, t4585, t4588, t4589, t4590, t4593)
}
