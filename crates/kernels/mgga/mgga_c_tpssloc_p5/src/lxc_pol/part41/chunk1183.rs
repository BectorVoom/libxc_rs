//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1183/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1183<F: Float>(t1102: F, t18761: F, t11137: F, t14818: F, t18227: F, t18239: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F) -> (F, F) {
    let t18762 = t18761 * t1102;
    let t18783 = F::cast_from(0.12077e1_f64) * t18227 + F::cast_from(0.36793333333333333333e-1_f64) * t14818 - F::cast_from(0.27595e-1_f64) * t18515 + F::cast_from(0.36793333333333333333e-1_f64) * t18497 + F::cast_from(0.16557e0_f64) * t18518 + F::cast_from(0.13418888888888888889e0_f64) * t11137 + F::cast_from(0.60385e0_f64) * t18239 - F::cast_from(0.5519e-1_f64) * t18503 - F::cast_from(0.16557e0_f64) * t18500 + F::cast_from(0.33114e0_f64) * t18510 + F::cast_from(0.49671e0_f64) * t18508;
    (t18762, t18783)
}
