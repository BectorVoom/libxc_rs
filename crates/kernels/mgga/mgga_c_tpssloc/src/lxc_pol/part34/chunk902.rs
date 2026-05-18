//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 902/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk902<F: Float>(t1557: F, t5726: F, t2792: F, t1556: F, t17520: F, t2842: F, t1569: F, t5758: F, t10636: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F, F, F) {
    let t21315 = t1557 * t5726;
    let t21317 = F::new(6.0) * t2792 * t21315;
    let t21318 = t17520 * t1556;
    let t21320 = F::new(0.48245938496077605201e2) * t2842 * t21318;
    let t21321 = t1569 * t5758;
    let t21334 = -t10636 - F::new(0.23744444444444444444e-1) * t13598 + F::new(0.11872222222222222222e-1) * t17149 - F::new(0.35616666666666666666e-1) * t17165 + F::new(0.17808333333333333333e-1) * t17175 - F::new(0.19787037037037037037e-1) * t21147 + F::new(0.71233333333333333332e-1) * t21150 - F::new(0.35616666666666666666e-1) * t21124 - F::new(0.10685e0) * t21153 + F::new(0.10685e0) * t21128 - F::new(0.17808333333333333333e-1) * t21156;
    (t21317, t21320, t21321, t21334)
}
