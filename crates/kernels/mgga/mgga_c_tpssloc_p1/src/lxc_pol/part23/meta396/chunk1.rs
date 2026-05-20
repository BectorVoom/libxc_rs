//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1202/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202<F: Float>(t41385: F, t5587: F, t16673: F, t2629: F, t2696: F, t118: F, t2375: F, t5522: F, t16710: F, t2663: F, t2517: F, t2658: F, t5392: F) -> (F, F, F, F, F, F) {
    let t58809 = t41385 * t5587;
    let t58811 = t16673 * t2629;
    let t58844 = t16673 * t2696;
    let t58972 = t5522 * t118 * t2375;
    let t58984 = t16710 * t2663;
    let t59013 = t2658 * t2517 * t5392;
    (t58809, t58811, t58844, t58972, t58984, t59013)
}
