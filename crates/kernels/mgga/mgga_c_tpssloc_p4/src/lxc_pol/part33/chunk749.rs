//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 749/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk749<F: Float>(t5: F, t1860: F, t1865: F, t6490: F, t7428: F, t7432: F, t7435: F, t7442: F, t7446: F, t112: F, t1874: F, t4028: F, t1458: F, t89: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7450 = piecewise3::<F>(t8, F::new(0.0), -t7428 * t1865 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t6490 * t7432 + t7435 * t1865 / F::new(3.0) - t1860 * t7442 / F::new(6.0) - t1860 * t7446 / F::new(6.0));
    let t7451 = t7450 * t112;
    let t7457 = F::new(2.0) * t4028 * t1874;
    let t7458 = t89 * t1458;
    (t7450, t7451, t7457, t7458)
}
