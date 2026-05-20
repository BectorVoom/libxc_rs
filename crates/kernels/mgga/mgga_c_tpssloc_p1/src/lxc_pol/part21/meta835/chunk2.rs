//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2964/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2964<F: Float>(t17919: F, t3070: F, t42488: F, t1022: F, t3966: F, t360: F, t1041: F, t10868: F, t248: F, t5685: F, t14134: F, t4644: F) -> (F, F, F, F) {
    let t61768 = t3070 * t42488 * t17919;
    let t61774 = t3966 * t1022;
    let t61775 = t61774 * t360;
    let t61782 = t1041 * t248 * t10868 * t5685;
    let t61784 = t4644 * t14134;
    (t61768, t61775, t61782, t61784)
}
