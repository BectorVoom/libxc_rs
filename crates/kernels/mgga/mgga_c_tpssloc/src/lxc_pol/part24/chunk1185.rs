//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1185/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1185<F: Float>(t225: F, t3166: F, t387: F, t345: F, t1922: F, t2966: F, t1920: F, t1049: F, t6703: F, t6706: F, t6710: F, t6769: F) -> (F, F, F, F, F, F, F, F) {
    let t23353 = t3166 * t225 * t387;
    let t23354 = t345 * t23353;
    let t23357 = t2966 * t1922;
    let t23359 = F::new(0.18277045187202515961e-2) * t1920 * t23357;
    let t23365 = t6703 * t1049;
    let t23366 = t23365 * t6706;
    let t23369 = t6710 * t225;
    let t23372 = t6769 * t225;
    (t23353, t23354, t23357, t23359, t23365, t23366, t23369, t23372)
}
