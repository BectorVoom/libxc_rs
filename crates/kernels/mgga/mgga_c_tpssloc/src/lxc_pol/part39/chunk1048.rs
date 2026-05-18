//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1048/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1048<F: Float>(t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F, t2576: F, t13005: F, t13007: F, t13010: F, t13014: F, t13017: F, t787: F, t9572: F, t9574: F, t9579: F, t9583: F) -> F {
    let t13020 = t2563 * t4138;
    let t13022 = t9546 * t4134;
    let t13025 = t118 * t794 * t4119;
    let t13027 = F::new(0.16666666666666666666e-2) * t2576 * t13025;
    let t13028 = -F::new(0.19999999999999999999e-1) * t13005 * t13007 - t9572 - F::new(0.12962962962962962962e-1) * t13010 - t13014 - F::new(0.11666666666666666666e-1) * t9574 + t9579 - F::new(0.16666666666666666666e-2) * t787 * t13017 + F::new(0.77777777777777777774e-2) * t13020 - F::new(0.52777777777777777776e-2) * t13022 + t13027 - t9583;
    t13028
}
