//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 222/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk222<F: Float>(t576: F, t577: F, t11: F, t2: F, t10: F, t3: F, t9: F, t16: F) -> (F, F, F, F, F, F, F, F) {
    let t580 = 1.0 + 0.45e1 * t576 * t577;
    let t581 = t2 * t11;
    let t582 = 0.174e1 * t581;
    let t583 = t10 * t3;
    let t584 = 1.0 / t583;
    let t586 = 0.174e1 * t9 * t584;
    let t587 = t9 * t2;
    let t588 = t587 * t16;
    (t580, t581, t582, t583, t584, t586, t587, t588)
}
