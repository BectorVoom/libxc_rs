//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1205/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1205<F: Float>(t1442: F, t1774: F, t1849: F, t1869: F, t1976: F, t28819: F, t28822: F, t28825: F, t28829: F, t28833: F, t28837: F, t28841: F, t28843: F, t28852: F, t28855: F, t28861: F, t28863: F, t28866: F, t4028: F, t5450: F, t5457: F, t6287: F, t652: F, t7451: F, t7472: F, t7670: F, t7681: F) -> F {
    let t28867 = -F::cast_from(2.0_f64) * t1442 * t7670 - F::cast_from(2.0_f64) * t1774 * t7451 + F::cast_from(2.0_f64) * t1849 * t7681 - t1869 * t6287 - t1976 * t5450 - F::cast_from(2.0_f64) * t1976 * t5457 - F::cast_from(2.0_f64) * t28852 * t652 - F::cast_from(4.0_f64) * t28855 * t652 - F::cast_from(4.0_f64) * t4028 * t7472 + t28819 + t28822 + t28825 + t28829 - t28833 + t28837 + t28841 + t28843 - t28861 - t28863 - t28866;
    t28867
}
