//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1972/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1972<F: Float>(t87403: F, t87405: F, t87414: F, t87425: F, t87432: F, t92679: F, t98818: F, t98820: F, t98822: F, t98824: F, t98826: F, t98828: F, t98830: F, t98833: F, t98836: F, t98838: F, t98842: F, t98844: F) -> F {
    let t101486 = F::new(119.0) / F::new(1728.0) * t87403 - F::cast_from(0.21083550404717759668e-2_f64) * t87405 + t92679 - t98818 / F::new(192.0) - t98820 / F::new(192.0) - t98822 / F::new(96.0) - t98824 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t98826 - F::new(35.0) / F::new(288.0) * t98828 + F::new(7.0) / F::new(144.0) * t98830 - t98833 / F::new(192.0) - t87414 - F::cast_from(0.56521858531796547194e-2_f64) * t98836 - F::cast_from(0.23739180583354549821e0_f64) * t87425 - F::cast_from(0.45217486825437237755e-1_f64) * t87432 - F::cast_from(0.33913115119077928317e-1_f64) * t98838 - F::cast_from(0.24223653656484234512e-2_f64) * t98842 + t98844 / F::new(96.0);
    t101486
}
