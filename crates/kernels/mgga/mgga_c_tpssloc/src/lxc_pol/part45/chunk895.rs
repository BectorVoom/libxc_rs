//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 895/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk895<F: Float>(t26989: F, t6962: F, t6883: F, t8612: F, t1386: F, t2016: F, t24082: F, t26224: F, t31147: F, t31646: F, t31649: F, t31651: F, t31653: F, t3758: F, t3882: F, t8627: F, t8637: F) -> (F, F) {
    let t31655 = t26989 * t6962;
    let t31662 = t6883 * t8612;
    let t31663 = F::cast_from(0.19190897446562641759e-1_f64) * t31662;
    let t31666 = -F::cast_from(0.16449340668482264365e-1_f64) * t31646 - t31147 + t31649 - F::cast_from(0.82246703342411321825e-2_f64) * t31651 - t31653 * t1386 - F::new(6.0) * t26224 * t31655 + F::new(2.0) * t3758 * t8627 + F::new(2.0) * t3882 * t8627 - t31663 - t24082 * t2016 - t3758 * t8637;
    (t31655, t31666)
}
