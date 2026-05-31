//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1480/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1480<F: Float>(t122824: F, t122826: F, t122829: F, t122831: F, t122834: F, t122837: F, t122839: F, t122841: F, t122844: F, t122846: F, t24969: F, t31287: F, t33192: F, t7801: F) -> F {
    let t125046 = t122824 + t31287 + t122826 + t122829 + t122831 + t122834 + t122837 + t122839 + t122841 + t122844 + t33192 + t122846 + F::cast_from(0.135e2_f64) * t24969 * t7801;
    t125046
}
