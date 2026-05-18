//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1407/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1407<F: Float>(t114992: F, t115009: F, t119700: F, t121258: F, t121271: F, t121279: F, t121775: F, t121782: F, t121818: F, t1877: F, t23788: F, t24191: F, t2522: F, t25921: F, t25928: F, t26563: F, t26744: F, t26756: F, t28: F, t31504: F, t33466: F, t6841: F, t6848: F, t7656: F, t89953: F) -> F {
    let t122042 = -F::new(3.0) * t26563 * t23788 * t121818 + t26756 * t119700 + F::new(3.0) / F::new(2.0) * t2522 * t33466 * t6841 - F::new(3.0) * t26756 * t89953 * t121258 - t1877 * t121782 * t6848 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t23788 * t121279 - F::new(3.0) / F::new(2.0) * t115009 * t25921 - t1877 * t26744 * t31504 / F::new(2.0) + t121271 * t25928 + t1877 * t121775 * t28 / F::new(2.0) - t1877 * t114992 * t7656 / F::new(2.0);
    t122042
}
