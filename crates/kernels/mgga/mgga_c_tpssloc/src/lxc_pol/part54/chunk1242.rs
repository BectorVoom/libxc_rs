//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1242/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1242<F: Float>(t5: F, t1437: F, t8513: F, t8824: F, t1409: F, t32331: F, t8308: F, t1433: F, t32338: F, t31860: F, t31864: F, t32346: F, t33669: F, t33677: F, t8663: F, t8825: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t34122 = t8513 * t8824 * t1437;
    let t34125 = t32331 * t1409;
    let t34126 = t8308 * t34125;
    let t34132 = t8513 * t32338 * t1433;
    let t34136 = piecewise3::<f64>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t33669 * t8825 + F::new(5.0) / F::new(12.0) * t31860 * t34122 + F::new(5.0) / F::new(18.0) * t31864 * t34126 - F::new(5.0) / F::new(72.0) * t33677 * t8825 - F::new(5.0) / F::new(36.0) * t8663 * t34132 + t32346);
    (t34122, t34126, t34132, t34136)
}
