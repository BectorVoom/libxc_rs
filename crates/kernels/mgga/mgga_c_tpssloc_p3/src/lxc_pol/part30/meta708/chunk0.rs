//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2336/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2336<F: Float>(t28017: F, t3941: F, t671: F, t20173: F, t28899: F, t1395: F, t5456: F, t1873: F, t20162: F, t6534: F, t26545: F, t33185: F) -> (F, F, F, F, F) {
    let t100927 = F::new(27.0) * t3941 * t28017 * t671;
    let t100929 = F::new(27.0) * t20173 * t28899;
    let t100930 = t1395 * t5456;
    let t100932 = F::new(27.0) * t100930 * t1873;
    let t100934 = F::new(0.135e2) * t20162 * t6534;
    let t100936 = F::new(54.0) * t33185 * t26545;
    (t100927, t100929, t100932, t100934, t100936)
}
