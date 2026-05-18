//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 500/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk500<F: Float>(t28: F, t1081: F, t3231: F, t3672: F, t517: F, t157: F, t3671: F, t182: F, t118: F, t521: F, t2375: F, t1294: F, t2371: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t3673 = t1081 * t1081;
    let t3679 = piecewise3::<f64>(t29, F::new(0.0), F::new(4.0) / F::new(9.0) * t3672 * t3673 + F::new(4.0) / F::new(3.0) * t517 * t3231);
    let t3681 = (t3671 + t3679) * t157;
    let t3683 = F::new(0.19751673498613801407e-1) * t3681 * t182;
    let t3684 = t521 * t118;
    let t3686 = F::new(0.10843581300301739842e-1) * t3684 * t2375;
    let t3688 = F::new(0.11696447245269292414e1) * t1294 * t2371;
    (t3673, t3681, t3683, t3686, t3688)
}
