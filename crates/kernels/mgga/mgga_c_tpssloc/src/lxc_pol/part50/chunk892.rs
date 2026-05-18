//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 892/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk892<F: Float>(t1922: F, t2966: F, t1920: F, t1049: F, t6703: F, t225: F, t6710: F, t6769: F, t134: F, t221: F, t1926: F) -> (F, F, F, F, F, F) {
    let t23357 = t2966 * t1922;
    let t23359 = F::new(0.18277045187202515961e-2) * t1920 * t23357;
    let t23365 = t6703 * t1049;
    let t23369 = t6710 * t225;
    let t23372 = t6769 * t225;
    let t23383 = t221 * t134;
    let t23384 = t1926 * t23383;
    (t23359, t23365, t23369, t23372, t23383, t23384)
}
