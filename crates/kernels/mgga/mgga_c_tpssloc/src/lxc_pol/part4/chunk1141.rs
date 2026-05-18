//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1141/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1141<F: Float>(t1102: F, t18761: F, t11137: F, t14818: F, t18227: F, t18239: F, t18497: F, t18500: F, t18503: F, t18508: F, t18510: F, t18515: F, t18518: F) -> (F, F) {
    let t18762 = t18761 * t1102;
    let t18783 = F::new(0.12077e1) * t18227 + F::new(0.36793333333333333333e-1) * t14818 - F::new(0.27595e-1) * t18515 + F::new(0.36793333333333333333e-1) * t18497 + F::new(0.16557e0) * t18518 + F::new(0.13418888888888888889e0) * t11137 + F::new(0.60385e0) * t18239 - F::new(0.5519e-1) * t18503 - F::new(0.16557e0) * t18500 + F::new(0.33114e0) * t18510 + F::new(0.49671e0) * t18508;
    (t18762, t18783)
}
