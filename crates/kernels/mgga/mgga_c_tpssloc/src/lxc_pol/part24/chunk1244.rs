//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1244/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1244<F: Float>(t10454: F, t6765: F, t10889: F, t3033: F, t6753: F, t10510: F, t6755: F, t11002: F, t23537: F, t10895: F, t23541: F, t23529: F, t3053: F, t10955: F, t1940: F, t354: F) -> (F, F, F, F, F, F, F) {
    let t82843 = t6765 * t10454;
    let t82848 = t3033 * t6753 * t10889;
    let t82851 = t6755 * t10510;
    let t82859 = t23537 * t11002;
    let t82861 = t23541 * t10895;
    let t82863 = t23529 * t3053;
    let t82868 = t354 * t1940 * t10955;
    (t82843, t82848, t82851, t82859, t82861, t82863, t82868)
}
