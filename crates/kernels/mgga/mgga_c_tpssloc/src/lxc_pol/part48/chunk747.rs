//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 747/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk747<F: Float>(t23253: F, t6562: F, t225: F, t258: F, t2710: F, t214: F, t1880: F, t1883: F, t23012: F, t23237: F, t6572: F, t213: F, t252: F) -> (F, F, F, F, F, F) {
    let t23254 = t6562 * t23253;
    let t23257 = t2710 * t225 * t258;
    let t23258 = t214 * t23257;
    let t23259 = t1880 * t23258;
    let t23261 = t23012 * t1883;
    let t23262 = F::cast_from(0.63969658155208805863e-1_f64) * t23261;
    let t23265 = t23237 * t6572;
    let t23266 = t1880 * t23265;
    let t23270 = t213 * t252 * t225;
    (t23254, t23259, t23261, t23262, t23266, t23270)
}
