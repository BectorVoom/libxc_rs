//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 913/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk913<F: Float>(t1902: F, t212: F, t23171: F, t6554: F, t794: F, t23164: F, t6555: F, t6562: F, t6572: F, t234: F, t6624: F, t6552: F, t6637: F, t776: F) -> (F, F, F, F) {
    let t112942 = F::new(0.16449340668482264365e-1) * t23171 * t212 * t1902 * t6554;
    let t112943 = t794 * t1902;
    let t112945 = t23164 * t112943 * t6555;
    let t112946 = F::new(0.3289868133696452873e-1) * t112945;
    let t112948 = t6562 * t112943 * t6572;
    let t112949 = F::new(0.16449340668482264365e-1) * t112948;
    let t112951 = t234 * t6624;
    let t112955 = F::new(0.6579736267392905746e-1) * t6552 * t6637 * t112951 * t776;
    (t112942, t112946, t112949, t112955)
}
