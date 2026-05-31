//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1999/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1999<F: Float>(t25: F, t265: F, t394: F, t93052: F, t93099: F, t12606: F, t1409: F, t2064: F, t2250: F, t24380: F, t26807: F, t3966: F, t40: F, t607: F, t7131: F, t7865: F, t92270: F, t92309: F, t92349: F, t93005: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t93100 = t93052 + t93099;
    let t93101 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t93100);
    let t93113 = piecewise3::<F>(t115, t92270 + t92309 + t92349 + t93005, t93101 * t40 / F::cast_from(2.0_f64) + t26807 * t607 + t7865 * t2250 / F::cast_from(2.0_f64) + t24380 * t1409 / F::cast_from(2.0_f64) + t7131 * t3966 + t2064 * t12606 / F::cast_from(2.0_f64));
    (t93100, t93113)
}
