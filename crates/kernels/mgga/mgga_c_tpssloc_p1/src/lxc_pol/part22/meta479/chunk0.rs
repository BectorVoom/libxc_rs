//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1879/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1879<F: Float>(t1495: F, t210: F, t5544: F, t10026: F, t10029: F, t13368: F, t16942: F, t16954: F, t16988: F, t16990: F, t16993: F, t16995: F, t17000: F, t2571: F) -> (F, F) {
    let t21008 = t210 * t1495 * t5544;
    let t21011 = F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t16942 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t16954 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t16988 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t16990 - t10026 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t16993 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t16995 - F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t17000 - t10029 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t13368 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2571 * t21008;
    (t21008, t21011)
}
