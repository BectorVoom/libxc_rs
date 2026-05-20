//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1407/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1407<F: Float>(t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43820: F, t43823: F, t43828: F) -> F {
    let t43831 = -F::new(80.0) / F::new(81.0) * t43811 + F::new(8.0) / F::new(9.0) * t43727 - F::new(8.0) / F::new(3.0) * t43729 + F::new(20.0) / F::new(9.0) * t43734 - F::new(112.0) / F::new(81.0) * t43816 + t43820 - F::new(8.0) * t43737 - F::new(2.0) / F::new(3.0) * t43823 - F::new(8.0) / F::new(9.0) * t43740 + F::new(12.0) * t43743 + F::new(2.0) * t43828 + F::new(8.0) / F::new(3.0) * t43746;
    t43831
}
