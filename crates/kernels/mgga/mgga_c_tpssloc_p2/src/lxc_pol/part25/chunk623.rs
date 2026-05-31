//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 623/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk623<F: Float>(t531: F, t571: F, t193: F, t2423: F, t2426: F, t2486: F, t3734: F, t3816: F, t3819: F, t3821: F, t3823: F, t3825: F, t3828: F, t3830: F, t3832: F, t3834: F, t3836: F) -> F {
    let t3924 = t531 * t571;
    let t3928 = F::cast_from(6.0_f64) * t193 * t3734 * t3924 - t2423 - t2426 - t2486 - t3816 + t3819 + t3821 - t3823 + t3825 + t3828 - t3830 - t3832 + t3834 + t3836;
    t3928
}
