//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2587/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2587<F: Float>(t3447: F, t44583: F, t461: F, t4729: F, t15418: F, t1714: F, t11571: F, t14736: F, t15419: F, t14165: F, t44505: F, t11557: F, t4889: F) -> (F, F, F, F, F, F) {
    let t52057 = t3447 * t44583 * t461 * t4729;
    let t52059 = t15418 * t1714;
    let t52061 = t3447 * t52059 * t11571;
    let t52064 = t3447 * t15419 * t14736;
    let t52066 = t44505 * t14165;
    let t52074 = t4889 * t11557;
    (t52057, t52059, t52061, t52064, t52066, t52074)
}
