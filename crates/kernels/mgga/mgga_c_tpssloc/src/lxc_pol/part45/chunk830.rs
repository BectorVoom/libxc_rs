//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 830/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk830<F: Float>(t112726: F, t23222: F, t30663: F, t6552: F, t1880: F, t23196: F, t6562: F, t82133: F, t8335: F, t23168: F, t30664: F, t214: F, t225: F, t23150: F, t258: F, t30643: F, t6547: F) -> (F, F, F, F, F, F, F) {
    let t112727 = 0.76763589786250567036e-1 * t112726;
    let t112730 = 0.3289868133696452873e-1 * t6552 * t30663 * t23222;
    let t112733 = 0.3289868133696452873e-1 * t1880 * t30663 * t23196;
    let t112741 = t6562 * t82133 * t8335;
    let t112742 = 0.16449340668482264365e-1 * t112741;
    let t112743 = t23168 * t30664;
    let t112744 = 0.15352717957250113407e0 * t112743;
    let t112759 = 0.16449340668482264365e-1 * t1880 * t214 * t23150 * t225 * t258;
    let t112760 = t6547 * t30643;
    (t112727, t112730, t112733, t112742, t112744, t112759, t112760)
}
