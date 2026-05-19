//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1224/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1224<F: Float>(t5: F, t1409: F, t31682: F, t8308: F, t1433: F, t31691: F, t8513: F, t12571: F, t8662: F, t7973: F, t8301: F, t2240: F, t31860: F, t31864: F, t33115: F, t33564: F, t8515: F, t8663: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t33567 = t31682 * t1409;
    let t33568 = t8308 * t33567;
    let t33572 = t8513 * t31691 * t1433;
    let t33669 = t12571 * t8662;
    let t33676 = t8301 * t7973;
    let t33677 = t2240 * t33676;
    let t33685 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t33669 * t8515 - F::new(5.0) / F::new(24.0) * t31860 * t33564 - F::new(5.0) / F::new(36.0) * t31864 * t33568 + F::new(5.0) / F::new(144.0) * t33677 * t8515 + F::new(5.0) / F::new(72.0) * t8663 * t33572 + F::new(5.0) / F::new(144.0) * t8663 * t33115);
    (t33568, t33572, t33669, t33676, t33677, t33685)
}
