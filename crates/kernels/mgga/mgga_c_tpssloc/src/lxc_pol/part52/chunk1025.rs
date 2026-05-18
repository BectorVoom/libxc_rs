//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1025/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1025<F: Float>(t3127: F, t381: F, t23602: F, t1011: F, t1615: F, t4594: F, t1014: F, t1023: F, t1022: F, t7593: F, t1060: F, t1945: F, t4649: F) -> (F, F, F, F) {
    let t25483 = t3127 * t381;
    let t25484 = t23602 * t25483;
    let t25485 = t1615 * t1011;
    let t25486 = t25485 * t4594;
    let t25487 = t25484 * t25486;
    let t25490 = t1014 * t381;
    let t25491 = t23602 * t25490;
    let t25492 = t25485 * t1023;
    let t25493 = t25491 * t25492;
    let t25496 = t7593 * t1022;
    let t25497 = t25496 * t1060;
    let t25499 = t1945 * t4649;
    (t25487, t25493, t25497, t25499)
}
