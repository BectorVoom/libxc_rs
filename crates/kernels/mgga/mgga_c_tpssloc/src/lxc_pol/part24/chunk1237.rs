//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1237/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1237<F: Float>(t6790: F, t82632: F, t6787: F, t1014: F, t82514: F, t3032: F, t360: F, t225: F, t23547: F, t23631: F, t974: F, t976: F, t984: F, t1009: F, t343: F, t25490: F) -> (F, F, F, F, F, F, F) {
    let t82633 = t82632 * t6790;
    let t82635 = t82632 * t6787;
    let t82637 = t82514 * t1014;
    let t82638 = t3032 * t360;
    let t82643 = t23547 * t225;
    let t82653 = t23631 * t974 * t976 * t984;
    let t82654 = t343 * t1009;
    let t82655 = t82654 * t25490;
    (t82633, t82635, t82637, t82638, t82643, t82653, t82655)
}
