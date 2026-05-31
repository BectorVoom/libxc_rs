//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 998/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk998<F: Float>(t113741: F, t114971: F, t114988: F, t114992: F, t115027: F, t115030: F, t1877: F, t1914: F, t23792: F, t23796: F, t23807: F, t23813: F, t24191: F, t24339: F, t2522: F, t25927: F, t26756: F, t28: F, t30974: F, t31430: F, t31434: F, t31496: F, t31502: F, t3231: F, t6841: F, t6848: F, t7114: F, t84797: F, t8566: F, t92271: F) -> F {
    let t115184 = F::cast_from(2.0_f64) * t92271 * t31502 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8566 * t23796 - t1877 * t7114 * t3231 * t1914 / F::cast_from(2.0_f64) + F::cast_from(6.0_f64) * t24191 * t25927 * t115030 + F::cast_from(3.0_f64) * t2522 * t31430 * t6841 - t1877 * t7114 * t113741 / F::cast_from(2.0_f64) + t1877 * t115027 * t23807 + t26756 * t25927 * t114988 + t1877 * t114971 * t28 / F::cast_from(2.0_f64) - t1877 * t114992 * t6848 + t1877 * t8566 * t3231 / F::cast_from(2.0_f64) - t1877 * t24339 * t30974 - t1877 * t31434 * t23813 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t8566 * t23792 - F::cast_from(3.0_f64) * t84797 * t31496;
    t115184
}
