//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2066;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta652<F: Float>(t7611: F, t82716: F, t25550: F, t82822: F, t23384: F, t25476: F, t25467: F, t25459: F, t7604: F, t82632: F, t25723: F, t88810: F, t1539: F, t6746: F, t82655: F, t14220: F, t7581: F, t25555: F, t25529: F, t6680: F, t1920: F, t2966: F, t7614: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t89310, t89327, t89329, t89360, t89362, t89366, t89369) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2066::<F>(t7611, t82716, t25550, t82822, t23384, t25476, t25467, t25459, t7604, t82632, t25723, t88810);
        let (t89395, t89399, t89421, t89429, t89431) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2067::<F>(t1539, t6746, t82655, t14220, t7581, t25555, t82822, t25529, t6680, t1920, t2966, t7614);
    (t89310, t89327, t89329, t89360, t89362, t89366, t89369, t89395, t89399, t89421, t89429, t89431)
}
