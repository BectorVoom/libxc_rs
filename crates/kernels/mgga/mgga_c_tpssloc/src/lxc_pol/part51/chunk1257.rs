//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1257/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1257<F: Float>(t1081: F, t115009: F, t119719: F, t121789: F, t121837: F, t1877: F, t24191: F, t24339: F, t2522: F, t25892: F, t25898: F, t25905: F, t25927: F, t25934: F, t25945: F, t31430: F, t31434: F, t31496: F, t33466: F, t33539: F, t7649: F, t8566: F, t92319: F) -> (F,) {
    let t122012 = t1877 * t33466 * t1081 / 2.0 - t1877 * t31434 * t25934 / 2.0 - t1877 * t24339 * t33539 / 2.0 + 3.0 * t24191 * t25927 * t121837 - t1877 * t31434 * t25945 / 2.0 - 3.0 / 2.0 * t92319 * t31496 - 3.0 / 2.0 * t115009 * t25898 + 3.0 / 2.0 * t2522 * t31430 * t7649 + 3.0 / 2.0 * t2522 * t8566 * t25905 + 3.0 * t121789 * t25892 - 3.0 / 2.0 * t24191 * t119719;
    (t122012,)
}
