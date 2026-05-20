//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1712/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1712<F: Float>(t25: F, t28: F, t4021: F, t645: F, t1437: F, t2307: F, t1409: F, t9321: F, t2291: F, t3966: F, t584: F, t9212: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t12585 = t4021 * t645;
    let t12588 = t1437 * t2307;
    let t12595 = t9321 * t1409;
    let t12598 = t2291 * t3966;
    let t12603 = F::new(2.0) * t584;
    let t12604 = F::new(6.0) * t9212;
    let t12606 = piecewise5::<F>(t26, F::new(0.0), t29, F::new(0.0), t12603 - t12604);
    (t12585, t12588, t12595, t12598, t12603, t12604, t12606)
}
