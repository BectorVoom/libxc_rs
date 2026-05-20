//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2145/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2145<F: Float>(t15394: F, t1714: F, t3439: F, t3447: F, t461: F, t4724: F, t697: F, t11554: F, t1706: F, t11545: F, t134: F, t4899: F, t4928: F) -> (F, F, F, F, F) {
    let t52100 = t15394 * t1714;
    let t52109 = t3447 * t697 * t3439 * t461 * t4724;
    let t52110 = F::cast_from(0.24691358024691358024e-3_f64) * t52109;
    let t52124 = t1706 * t11554;
    let t52133 = t134 * t11545 * t461;
    let t52140 = t4899 * t4928;
    (t52100, t52110, t52124, t52133, t52140)
}
