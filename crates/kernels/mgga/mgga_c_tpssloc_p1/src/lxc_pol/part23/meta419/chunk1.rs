//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1244/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244<F: Float>(t21131: F, t699: F, t21135: F, t21139: F, t21119: F, t21697: F, t3216: F, t21238: F, t2929: F, t21334: F, t892: F, t21347: F, t300: F) -> (F, F, F, F, F, F, F, F) {
    let t68500 = t699 * t21131;
    let t68502 = t699 * t21135;
    let t68504 = t699 * t21139;
    let t68506 = t699 * t21119;
    let t68711 = t21697 * t3216;
    let t68902 = t2929 * t21238;
    let t68924 = t21334 * t892;
    let t69012 = t300 * t21347;
    (t68500, t68502, t68504, t68506, t68711, t68902, t68924, t69012)
}
