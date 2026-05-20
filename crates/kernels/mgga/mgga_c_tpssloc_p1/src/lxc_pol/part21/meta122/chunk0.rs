//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 824/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk824<F: Float>(t2770: F, t344: F, t2244: F, t2979: F, t337: F, t39: F, t1887: F) -> (F, F, F, F, F) {
    let t2980 = t344 * t2770;
    let t2981 = t2980 * t2244;
    let t2982 = t2979 * t2981;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    (t2980, t2981, t2982, t2985, t2986)
}
