//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 401/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk401<F: Float>(t1851: F, t3: F, t1401: F, t1458: F, t577: F, t33: F, t605: F) -> (F, F, F) {
    let t1852 = t3 * t1851;
    let t1858 = F::cast_from(0.45e1_f64) * t1851 * t577 + F::cast_from(0.135e2_f64) * t1401 * t1458;
    let t1860 = t605 * t33;
    (t1852, t1858, t1860)
}
