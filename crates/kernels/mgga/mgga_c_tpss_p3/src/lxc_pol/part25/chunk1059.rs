//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1059/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1059<F: Float>(t10980: F, t11003: F, t11005: F, t11006: F, t14459: F, t14492: F, t14495: F, t14505: F, t14507: F, t14517: F, t14521: F, t14525: F, t14528: F, t14532: F, t14535: F, t8616: F, t8687: F) -> F {
    let t14538 = -t8687 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8616 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10980 + t11003 - t11005 + t11006 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14495 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t14517 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14459 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14521 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14505 - F::cast_from(2.0_f64) * t14525 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t14528 + t14507 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14532 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14535 - t14492 / F::cast_from(3.0_f64);
    t14538
}
