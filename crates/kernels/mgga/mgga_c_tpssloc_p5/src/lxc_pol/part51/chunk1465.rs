//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1465/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1465<F: Float>(t120897: F, t120921: F, t120951: F, t120980: F, t121017: F, t121149: F, t121180: F, t121205: F, t121229: F, t122085: F, t122596: F, t122613: F, t122643: F, t122673: F, t122701: F, t122761: F) -> F {
    let t122765 = t120897 + t120921 + t120951 + t120980 + t121017 + t121149 + t121180 + t121205 + t121229 + t122085 + t122596 + F::cast_from(2.0_f64) * t122613 + t122643 + t122673 + t122701 + t122761;
    t122765
}
