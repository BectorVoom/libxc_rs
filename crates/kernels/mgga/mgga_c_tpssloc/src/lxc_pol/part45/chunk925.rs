//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 925/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk925<F: Float>(t6562: F, t82133: F, t8335: F, t23168: F, t30664: F, t1880: F, t214: F, t225: F, t23150: F, t258: F, t30643: F, t6547: F) -> (F, F, F, F) {
    let t112741 = t6562 * t82133 * t8335;
    let t112742 = F::cast_from(0.16449340668482264365e-1_f64) * t112741;
    let t112743 = t23168 * t30664;
    let t112744 = F::cast_from(0.15352717957250113407e0_f64) * t112743;
    let t112759 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t214 * t23150 * t225 * t258;
    let t112760 = t6547 * t30643;
    (t112742, t112744, t112759, t112760)
}
