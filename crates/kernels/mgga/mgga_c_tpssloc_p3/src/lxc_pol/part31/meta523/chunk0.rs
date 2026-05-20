//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1737/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1737<F: Float>(t29210: F, t29394: F, t3: F, t1458: F, t7801: F, t2039: F, t5493: F, t1401: F, t16524: F, t20162: F, t24465: F, t27254: F, t28893: F, t28951: F, t3941: F, t5371: F, t5456: F, t577: F, t7230: F, t7956: F) -> (F, F, F, F, F) {
    let t29395 = t29210 + t29394;
    let t29396 = t3 * t29395;
    let t29422 = t7801 * t1458;
    let t29425 = t2039 * t5493;
    let t29430 = F::new(0.45e1) * t29395 * t577 + F::new(27.0) * t27254 * t1458 + F::new(27.0) * t24465 * t5456 + F::new(0.135e2) * t7230 * t5493 + F::new(0.135e2) * t20162 * t2039 + F::new(54.0) * t16524 * t7956 + F::new(27.0) * t5371 * t7801 + F::new(27.0) * t28893 * t2039 + F::new(54.0) * t3941 * t29422 + F::new(27.0) * t3941 * t29425 + F::new(0.135e2) * t1401 * t28951;
    (t29395, t29396, t29422, t29425, t29430)
}
