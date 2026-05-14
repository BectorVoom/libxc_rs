//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 875/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk875<F: Float>(t6547: F, t6568: F, t23030: F, t6563: F, t6567: F, t794: F, t6562: F, t1883: F, t23012: F, t213: F, t225: F, t252: F) -> (F, F, F, F, F, F, F, F) {
    let t23249 = t6547 * t6568;
    let t23250 = 0.38381794893125283518e-1 * t23249;
    let t23251 = t23030 * t6563;
    let t23252 = 0.26044789391763585244e-1 * t23251;
    let t23253 = t794 * t6567;
    let t23254 = t6562 * t23253;
    let t23261 = t23012 * t1883;
    let t23262 = 0.63969658155208805863e-1 * t23261;
    let t23270 = t213 * t252 * t225;
    (t23249, t23250, t23251, t23252, t23254, t23261, t23262, t23270)
}
