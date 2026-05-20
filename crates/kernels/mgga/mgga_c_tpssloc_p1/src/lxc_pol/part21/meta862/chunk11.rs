//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3140/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3140<F: Float>(t1174: F, t15281: F, t18563: F, t18321: F, t3432: F, t11529: F, t6130: F, t15282: F, t4889: F, t18558: F, t3431: F, t12652: F, t14730: F) -> (F, F, F, F, F, F) {
    let t64969 = t1174 * t15281 * t18563;
    let t64976 = t18321 * t3432;
    let t64979 = t1174 * t11529 * t6130;
    let t64981 = t4889 * t15282;
    let t64988 = t1174 * t3431 * t18558;
    let t64990 = t14730 * t12652;
    (t64969, t64976, t64979, t64981, t64988, t64990)
}
