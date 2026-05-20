//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2646/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2646<F: Float>(t1851: F, t2319: F, t4072: F, t12521: F, t12524: F, t12813: F, t1458: F, t16535: F, t19534: F, t20181: F, t2363: F, t3941: F, t45560: F, t5371: F, t5376: F, t5456: F, t5493: F, t55341: F, t55353: F, t55388: F, t671: F) -> (F, F) {
    let t55405 = t1851 * t2319;
    let t55410 = t4072 * t4072;
    let t55417 = F::new(27.0) * t55388 * t2319 + F::new(27.0) * t5371 * t12813 + F::new(27.0) * t55341 * t1458 + F::new(0.135e2) * t12521 * t5493 + F::new(54.0) * t3941 * t19534 * t671 + F::new(27.0) * t3941 * t5493 * t2363 + F::new(54.0) * t12524 * t20181 + F::new(54.0) * t55405 * t1458 + F::new(27.0) * t16535 * t5493 + F::new(54.0) * t3941 * t55410 + F::new(108.0) * t55353 * t5376 + F::new(27.0) * t45560 * t5456;
    (t55410, t55417)
}
