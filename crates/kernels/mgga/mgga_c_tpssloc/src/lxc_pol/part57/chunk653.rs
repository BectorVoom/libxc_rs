//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 653/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk653<F: Float>(t23631: F, t23632: F, t381: F, t883: F, t6743: F, t6796: F, t995: F, t23602: F, t3127: F, t1011: F, t3131: F, t225: F, t23592: F, t11094: F, t1958: F, t2752: F, t28: F) -> (F, F, F, F, F, F, F, F) {
    let t23633 = t23631 * t23632;
    let t23634 = t381 * t883;
    let t23635 = t6743 * t23634;
    let t23665 = t6796 * t995;
    let t23677 = t23602 * t3127;
    let t23678 = t1011 * t3131;
    let t23696 = t23592 * t225;
    let t23742 = t1958 * t11094;
    let t23788 = t2752 * t28;
    (t23633, t23635, t23665, t23677, t23678, t23696, t23742, t23788)
}
