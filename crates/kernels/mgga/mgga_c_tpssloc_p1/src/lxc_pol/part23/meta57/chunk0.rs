//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 349/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk349<F: Float>(t40: F, t52: F, t1458: F, t510: F, t1409: F, t185: F, t707: F, t73: F, t76: F, zeta_threshold: F) -> (F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t1459 = t510 * t1458;
    let t1462 = t185 * t1409;
    let t1464 = F::new(4.0) * t707 * t1462;
    let t1467 = piecewise3::<F>(t146, F::new(0.0), F::new(4.0) / F::new(3.0) * t73 * t1409);
    let t1470 = piecewise3::<F>(t150, F::new(0.0), -F::new(4.0) / F::new(3.0) * t76 * t1409);
    let t1471 = t1467 + t1470;
    (t1459, t1462, t1464, t1471)
}
