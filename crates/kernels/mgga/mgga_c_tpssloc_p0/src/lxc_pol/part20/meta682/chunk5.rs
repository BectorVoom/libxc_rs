//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2579/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2579<F: Float>(t11583: F, t12652: F, t12648: F, t11570: F, t14165: F, t44607: F, t10913: F, t4723: F, t11536: F, t4889: F, t1174: F, t15268: F, t15281: F) -> (F, F, F, F, F, F, F) {
    let t52216 = t11583 * t12652;
    let t52220 = t11583 * t12648;
    let t52224 = t11570 * t14165;
    let t52228 = t44607 * t14165;
    let t52236 = t4723 * t10913;
    let t52240 = t4889 * t11536;
    let t52250 = t1174 * t15281 * t15268;
    (t52216, t52220, t52224, t52228, t52236, t52240, t52250)
}
