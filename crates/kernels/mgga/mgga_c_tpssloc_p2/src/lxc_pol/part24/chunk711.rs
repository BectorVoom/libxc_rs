//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 711/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk711<F: Float>(t5: F, t1860: F, t1865: F, t6486: F, t6490: F, t6492: F, t6495: F, t6506: F, t6510: F, t112: F, t111: F, t1868: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t6514 = piecewise3::<F>(t8, F::new(0.0), -t6486 * t1865 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t6490 * t6492 + t6495 * t1865 / F::new(3.0) - t1860 * t6506 / F::new(6.0) - t1860 * t6510 / F::new(6.0));
    let t6515 = t6514 * t112;
    let t6517 = t1868 * t111;
    (t6514, t6515, t6517)
}
