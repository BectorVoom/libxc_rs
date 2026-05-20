//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1872/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1872<F: Float>(t2148: F, t4930: F, t1716: F, t7381: F, t3502: F, t491: F, t24813: F, t1011: F, t1734: F, t4978: F, t1209: F, t1216: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27481 = t4930 * t2148;
    let t27484 = t1716 * t7381;
    let t27488 = t3502 * t491;
    let t27489 = t24813 * t27488;
    let t27490 = t1734 * t1011;
    let t27491 = t27490 * t4978;
    let t27492 = t27489 * t27491;
    let t27495 = t1209 * t491;
    let t27496 = t24813 * t27495;
    let t27497 = t27490 * t1216;
    (t27481, t27484, t27488, t27489, t27490, t27491, t27492, t27495, t27496, t27497)
}
