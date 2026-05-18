//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 991/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk991<F: Float>(t1495: F, t210: F, t5544: F, t10026: F, t10029: F, t13368: F, t16942: F, t16954: F, t16988: F, t16990: F, t16993: F, t16995: F, t17000: F, t2571: F) -> F {
    let t21008 = t210 * t1495 * t5544;
    let t21011 = F::new(7.0) / F::new(1536.0) * t16942 + F::new(7.0) / F::new(384.0) * t16954 - F::new(35.0) / F::new(384.0) * t16988 + F::new(7.0) / F::new(192.0) * t16990 - t10026 - F::new(7.0) / F::new(16.0) * t16993 + F::new(7.0) / F::new(48.0) * t16995 - F::new(7.0) / F::new(1536.0) * t17000 - t10029 - F::new(119.0) / F::new(1152.0) * t13368 + F::new(3.0) / F::new(16.0) * t2571 * t21008;
    t21011
}
