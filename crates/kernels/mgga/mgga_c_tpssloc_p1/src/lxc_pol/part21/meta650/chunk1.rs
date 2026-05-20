//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2446/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2446<F: Float>(t2955: F, t3158: F, t10383: F, t964: F, t1020: F, t10508: F, t248: F, t3121: F, t10868: F, t820: F, t3070: F, t3072: F) -> (F, F, F, F, F) {
    let t43155 = t2955 * t3158;
    let t43157 = t964 * t10383;
    let t43161 = t1020 * t248 * t10508 * t3121;
    let t43198 = t820 * t10868;
    let t43200 = t3070 * t43198 * t3072;
    (t43155, t43157, t43161, t43198, t43200)
}
