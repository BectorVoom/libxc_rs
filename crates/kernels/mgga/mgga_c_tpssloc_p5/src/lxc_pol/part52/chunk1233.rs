//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1233/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1233<F: Float>(t33702: F, t33722: F, t33743: F, t33759: F, t3: F, t1873: F, t27921: F, t24972: F, t7769: F, t7423: F, t7467: F, t1458: F, t31937: F, t33177: F, t33179: F, t33181: F, t33184: F, t33187: F, t33190: F, t33192: F, t33195: F, t577: F, t8508: F) -> (F, F, F) {
    let t33761 = t33702 + t33722 + t33743 + t33759;
    let t33762 = t3 * t33761;
    let t33774 = t27921 * t1873;
    let t33776 = t24972 * t7769;
    let t33778 = t7423 * t7467;
    let t33783 = F::new(0.45e1) * t33761 * t577 + F::new(0.135e2) * t31937 * t1458 + F::new(0.135e2) * t33774 + F::new(27.0) * t33776 + F::new(0.135e2) * t33778 + F::new(0.135e2) * t33177 + F::new(27.0) * t33179 + F::new(0.135e2) * t33181 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    (t33761, t33762, t33783)
}
