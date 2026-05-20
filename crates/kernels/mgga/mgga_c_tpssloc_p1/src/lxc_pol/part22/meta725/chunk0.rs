//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2378/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2378<F: Float>(t21131: F, t699: F, t21135: F, t21139: F, t21119: F, t5705: F, t896: F, t13634: F, t13637: F, t21510: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t68500 = t699 * t21131;
    let t68502 = t699 * t21135;
    let t68504 = t699 * t21139;
    let t68506 = t699 * t21119;
    let t68508 = t5705 * t896;
    let t68509 = t13634 * t68508;
    let t68511 = t13637 * t68508;
    let t68513 = t21510 * t607;
    (t68500, t68502, t68504, t68506, t68509, t68511, t68513)
}
