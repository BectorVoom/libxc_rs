//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 866/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk866<F: Float>(t77906: F, t69166: F, t14451: F, t1587: F, t5259: F, t321: F, t4669: F, t77883: F, t77884: F, t77887: F, t77888: F, t77889: F, t77890: F, t77894: F, t77898: F, t77899: F, t77900: F, t77904: F) -> (F,) {
    let t77907 = 0.44903406381989282115e-1 * t77906;
    let t77908 = 0.79828278012425390427e-1 * t69166;
    let t77910 = t5259 * t14451 * t1587;
    let t77911 = 0.2993560425465952141e-1 * t77910;
    let t77912 = -t77883 - t77884 + t77887 + t77888 + t77889 - 0.17961362552795712846e0 * t4669 * t77890 * t321 + 0.11974241701863808564e0 * t5259 * t77894 * t321 - t77898 + t77899 + t77900 - t77904 + t77907 + t77908 - t77911;
    (t77912,)
}
