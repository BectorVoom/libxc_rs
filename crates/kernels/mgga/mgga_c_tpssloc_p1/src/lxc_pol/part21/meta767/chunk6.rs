//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2651/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2651<F: Float>(t109: F, t55530: F, t55566: F, t2363: F, t576: F, t12649: F, t12652: F, t12653: F, t12656: F, t12661: F, t12708: F, t1410: F, t1426: F, t1434: F, t19343: F, t19346: F, t19349: F, t19441: F, t2304: F, t3961: F, t3962: F, t3967: F, t3997: F, t4018: F, t5403: F, t609: F, t642: F, t80: F) -> (F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t55568 = piecewise3::<F>(t110, F::new(0.0), t55530 + t55566);
    let t55571 = t576 * t2363;
    let t55631 = -t12652 * t1426 * t80 / F::new(3.0) - t3961 * t3997 * t80 / F::new(3.0) - t19343 * t642 / F::new(3.0) - t12661 * t1426 * t80 / F::new(6.0) - t3967 * t3997 * t80 / F::new(3.0) - t19346 * t642 / F::new(3.0) - t1410 * t12708 * t80 / F::new(6.0) - t19349 * t642 / F::new(3.0) - t5403 * t2304 / F::new(6.0) - t609 * t19441 / F::new(6.0) - t12649 * t1434 / F::new(6.0) - t12653 * t1434 / F::new(3.0) - t12656 * t1434 / F::new(3.0) - t3962 * t4018 / F::new(3.0);
    (t55568, t55571, t55631)
}
