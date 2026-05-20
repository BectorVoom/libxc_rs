//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1152/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1152<F: Float>(t10021: F, t812: F, t815: F, t2628: F, t2690: F, t835: F, t9972: F, t6589: F, t67: F, t246: F, t22715: F, t268: F, t271: F) -> (F, F, F, F, F) {
    let t41362 = t812 * t815 * t10021;
    let t41385 = t812 * t2628 * t2690;
    let t41414 = t812 * t9972 * t835;
    let t41466 = t6589 * t67;
    let t41467 = t41466 * t246;
    let t41654 = t268 * t22715 * t271;
    (t41362, t41385, t41414, t41467, t41654)
}
