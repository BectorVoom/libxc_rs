//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 580/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk580<F: Float>(t1557: F, t2787: F, t912: F, t2792: F, t1547: F, t2798: F, t896: F, t2766: F, t2802: F, t4335: F, t4340: F, t4345: F, t4349: F) -> (F, F, F, F) {
    let t4358 = F::new(1.0) * t2787 * t1557;
    let t4359 = t1557 * t912;
    let t4361 = F::new(2.0) * t2792 * t4359;
    let t4362 = t2798 * t1547;
    let t4363 = t4362 * t896;
    let t4370 = t2802 + t2766 / F::new(9.0) + t4335 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t4340 + F::new(2.0) / F::new(3.0) * t4345 - t4349 / F::new(3.0);
    (t4358, t4361, t4363, t4370)
}
