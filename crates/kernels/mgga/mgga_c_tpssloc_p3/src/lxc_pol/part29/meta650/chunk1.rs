//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2170/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2170<F: Float>(t25927: F, t86781: F, t1877: F, t1915: F, t22959: F, t23286: F, t23290: F, t25013: F, t2522: F, t25928: F, t25945: F, t28: F, t6670: F, t7649: F, t86703: F, t86734: F, t86751: F, t86757: F, t87945: F, t89881: F, t89888: F, t89892: F, t89896: F, t89904: F, t89907: F, t89911: F) -> F {
    let t89917 = t25927 * t86781;
    let t89920 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t89881 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t23286 * t7649 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t89888 + F::cast_from(3.0_f64) * t2522 * t1915 * t89892 + F::cast_from(6.0_f64) * t25013 * t89896 + t1877 * t87945 * t28 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t86703 * t25928 + t86734 + F::cast_from(3.0_f64) * t22959 * t89904 + t86751 - t1877 * t6670 * t89907 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t89911 - t86757 - t1877 * t23290 * t25945 + F::cast_from(6.0_f64) * t22959 * t89917;
    t89920
}
