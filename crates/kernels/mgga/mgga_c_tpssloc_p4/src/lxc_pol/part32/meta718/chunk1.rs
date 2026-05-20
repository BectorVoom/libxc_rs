//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2284/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2284<F: Float>(t25994: F, t7458: F, t28817: F, t6876: F, t1983: F, t28826: F, t83859: F, t26149: F, t7685: F, t16524: F, t26545: F, t1873: F, t66958: F) -> (F, F, F, F, F, F) {
    let t100840 = F::new(4.0) * t7458 * t25994;
    let t100854 = F::new(6.0) * t6876 * t28817;
    let t100861 = F::new(6.0) * t1983 * t83859 * t28826;
    let t100863 = F::new(2.0) * t7685 * t26149;
    let t100871 = F::new(54.0) * t16524 * t26545;
    let t100873 = F::new(0.135e2) * t66958 * t1873;
    (t100840, t100854, t100861, t100863, t100871, t100873)
}
