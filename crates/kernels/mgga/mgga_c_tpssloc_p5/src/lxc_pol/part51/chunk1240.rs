//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1240/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1240<F: Float>(t5: F, t12571: F, t8511: F, t1437: F, t8513: F, t8514: F, t1409: F, t31682: F, t8308: F, t1433: F, t31691: F, t31675: F, t31681: F, t31690: F, t33115: F, t8512: F, t8515: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t33560 = t12571 * t8511;
    let t33564 = t8513 * t8514 * t1437;
    let t33567 = t31682 * t1409;
    let t33568 = t8308 * t33567;
    let t33572 = t8513 * t31691 * t1433;
    let t33578 = piecewise3::<F>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t33560 * t8515 + F::new(5.0) / F::new(12.0) * t31675 * t33564 + F::new(5.0) / F::new(18.0) * t31681 * t33568 + t31690 - F::new(5.0) / F::new(36.0) * t8512 * t33572 - F::new(5.0) / F::new(72.0) * t8512 * t33115);
    (t33560, t33564, t33568, t33572, t33578)
}
