//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 957/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk957<F: Float>(t344: F, t381: F, t225: F, t1054: F, t883: F, t6733: F, t6686: F, t6712: F, t1922: F, t2966: F, t1920: F, t1049: F, t6703: F) -> (F, F, F, F, F, F) {
    let t23328 = t344 * t381;
    let t23329 = t23328 * t225;
    let t23330 = t1054 * t883;
    let t23336 = t6733 * t381;
    let t23346 = t6712 * t6686;
    let t23357 = t2966 * t1922;
    let t23359 = F::new(0.18277045187202515961e-2) * t1920 * t23357;
    let t23365 = t6703 * t1049;
    (t23329, t23330, t23336, t23346, t23359, t23365)
}
