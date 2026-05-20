//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2091/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2091<F: Float>(t23460: F, t995: F, t23452: F, t6739: F, t6741: F, t23482: F, t23488: F, t23508: F, t6721: F, t1937: F, t23453: F, t40: F) -> (F, F, F, F, F) {
    let t83098 = t23460 * t995;
    let t83111 = t23452 * t6739 * t6741;
    let t83114 = t23482 * t23488;
    let t83120 = t6721 * t23508;
    let t83127 = t23453 * t40 * t1937;
    (t83098, t83111, t83114, t83120, t83127)
}
