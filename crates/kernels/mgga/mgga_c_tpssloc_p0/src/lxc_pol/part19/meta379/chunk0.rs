//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1416/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1416<F: Float>(t43819: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F) -> F {
    let t43942 = F::cast_from(0.96141975308641975307e-1_f64) * t43819;
    let t43949 = -F::cast_from(0.27469135802469135803e-1_f64) * t43811 + F::cast_from(0.24722222222222222222e-1_f64) * t43727 - F::cast_from(0.74166666666666666668e-1_f64) * t43729 + F::cast_from(0.61805555555555555555e-1_f64) * t43734 - F::cast_from(0.38456790123456790123e-1_f64) * t43816 + t43942 - F::cast_from(0.22249999999999999999e0_f64) * t43737 - F::cast_from(0.18541666666666666666e-1_f64) * t43823 - F::cast_from(0.24722222222222222222e-1_f64) * t43740 + F::new(0.33375e0) * t43743 + F::cast_from(0.55625000000000000001e-1_f64) * t43828 + F::cast_from(0.74166666666666666668e-1_f64) * t43746;
    t43949
}
