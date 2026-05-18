//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 796/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk796<F: Float>(t1011: F, t3507: F, t3508: F, t24661: F, t1209: F, t3030: F, t478: F, t475: F, t1222: F, t7334: F, t2140: F, t3566: F) -> (F, F, F, F) {
    let t24662 = t3507 * t1011;
    let t24663 = t24662 * t3508;
    let t24664 = t24661 * t24663;
    let t24667 = t3030 * t1209;
    let t24668 = t24667 * t478;
    let t24669 = t24662 * t475;
    let t24670 = t24668 * t24669;
    let t24675 = t7334 * t1222;
    let t24677 = t3566 * t2140;
    (t24664, t24670, t24675, t24677)
}
