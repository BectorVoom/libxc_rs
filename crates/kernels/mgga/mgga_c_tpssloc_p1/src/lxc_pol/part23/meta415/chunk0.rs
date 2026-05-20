//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1233/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1233<F: Float>(t120: F, t20800: F, t20904: F, t41414: F, t20949: F, t2697: F, t20882: F, t9638: F, t13258: F, t20988: F, t20887: F, t20969: F, t2639: F) -> (F, F, F, F, F, F, F) {
    let t67644 = t120 * t20800;
    let t67660 = t41414 * t20904;
    let t67675 = t2697 * t20949;
    let t67690 = t9638 * t20882;
    let t67692 = t13258 * t20988;
    let t67729 = t9638 * t20887;
    let t67735 = t2639 * t20969;
    (t67644, t67660, t67675, t67690, t67692, t67729, t67735)
}
