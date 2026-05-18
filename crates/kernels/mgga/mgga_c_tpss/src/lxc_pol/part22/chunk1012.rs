//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1012/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1012<F: Float>(t10952: F, t912: F, t2629: F, t3909: F, t1485: F, t9133: F, t3762: F, t845: F, t867: F, t2526: F, t3765: F, t1411: F, t2530: F) -> (F, F, F, F, F, F) {
    let t10954 = F::new(0.11696447245269292414e1) * t912 * t10952;
    let t10956 = F::new(0.34631718211362927518e2) * t2629 * t3909;
    let t10957 = t1485 * t9133;
    let t10961 = t3762 * t845;
    let t10963 = F::new(2.0) * t10961 * t867;
    let t10965 = F::new(1.0) * t3765 * t2526;
    let t10966 = t1411 * t2530;
    (t10954, t10956, t10957, t10963, t10965, t10966)
}
