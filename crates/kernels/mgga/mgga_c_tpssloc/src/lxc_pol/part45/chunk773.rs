//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 773/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk773<F: Float>(t10165: F, t23340: F, t6686: F, t6712: F, t225: F, t3166: F, t387: F, t345: F, t1922: F, t2966: F, t1920: F, t1049: F, t6703: F) -> (F, F, F, F, F) {
    let t23341 = t10165 * t23340;
    let t23346 = t6712 * t6686;
    let t23353 = t3166 * t225 * t387;
    let t23354 = t345 * t23353;
    let t23357 = t2966 * t1922;
    let t23359 = F::new(0.18277045187202515961e-2) * t1920 * t23357;
    let t23365 = t6703 * t1049;
    (t23341, t23346, t23354, t23359, t23365)
}
