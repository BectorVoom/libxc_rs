//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 817/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk817<F: Float>(t24230: F, t24231: F, t25109: F, t25126: F, t25133: F, t25140: F, t25144: F, t28380: F, t28384: F, t28386: F, t28390: F, t28397: F, t28399: F, t28401: F, t28403: F) -> F {
    let t29039 = F::cast_from(0.33913115119077928316e-1_f64) * t25109 + t28380 / F::new(96.0) - F::cast_from(0.24223653656484234512e-2_f64) * t28384 + t28386 / F::new(8.0) + F::cast_from(0.16956557559538964158e-1_f64) * t28390 + F::cast_from(0.56521858531796547194e-2_f64) * t25126 + F::cast_from(0.13457585364713463618e-3_f64) * t25133 + F::cast_from(0.48447307312968469024e-2_f64) * t28397 + F::new(7.0) / F::new(36.0) * t25140 - t28399 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t28401 + F::new(7.0) / F::new(576.0) * t25144 - t28403 / F::new(24.0) + t24230 + t24231;
    t29039
}
