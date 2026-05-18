//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 848/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk848<F: Float>(t761: F, t9905: F, t2509: F, t746: F, t9490: F, t1891: F, t68: F, t813: F) -> (F, F, F, F, F) {
    let t9907 = F::new(0.35089341735807877242e1) * t761 * t9905;
    let t9919 = t2509 * t9490 * t746;
    let t9921 = F::new(0.35089341735807877242e1) * t761 * t9919;
    let t9946 = t68 * t1891;
    let t9970 = t813 * t813;
    let t9971 = F::new(1.0) / t9970;
    (t9907, t9919, t9921, t9946, t9971)
}
