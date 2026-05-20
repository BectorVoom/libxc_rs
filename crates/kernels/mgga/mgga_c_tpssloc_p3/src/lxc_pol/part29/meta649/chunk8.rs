//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2166/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2166<F: Float>(t12971: F, t13196: F, t1484: F, t1877: F, t1915: F, t23286: F, t23290: F, t23295: F, t2522: F, t25354: F, t25358: F, t2745: F, t2749: F, t4255: F, t4303: F, t4314: F, t47645: F, t57921: F, t58009: F, t58071: F, t59580: F, t6666: F, t6670: F, t7634: F, t776: F, t86706: F, t86713: F, t86815: F, t87975: F) -> F {
    let t89822 = F::new(3.0) * t12971 * t1915 * t2522 + F::new(6.0) * t13196 * t1915 * t4314 + F::new(3.0) * t1484 * t23286 * t2522 - F::new(2.0) * t1877 * t23290 * t4303 + F::new(4.0) * t1877 * t23295 * t58009 + F::new(2.0) * t1877 * t23295 * t86713 - t1877 * t25358 * t2745 + F::new(2.0) * t1877 * t2749 * t87975 + F::new(6.0) * t23295 * t2522 * t57921 + F::new(6.0) * t2522 * t25354 * t776 - F::new(6.0) * t2522 * t58071 * t6670 - F::new(3.0) * t2522 * t59580 * t6670 - F::new(3.0) * t2522 * t6670 * t86815 + F::new(12.0) * t4255 * t4314 * t6666 - F::new(6.0) * t4314 * t6670 * t86706 + F::new(6.0) * t47645 * t7634;
    t89822
}
