//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1304/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1304<F: Float>(t24462: F, t7467: F, t122824: F, t122826: F, t122829: F, t122831: F, t122834: F, t122837: F, t122839: F, t122841: F, t122844: F, t26523: F, t31287: F, t33192: F, t7056: F, t1851: F, t8660: F) -> (F, F) {
    let t122846 = 0.135e2 * t24462 * t7467;
    let t122847 = 0.135e2 * t26523 * t7056 + t122824 + t31287 + t122826 + t122829 + t122831 + t122834 + t122837 + t122839 + t122841 + t122844 + t33192 + t122846;
    let t122852 = t1851 * t8660;
    (t122847, t122852)
}
