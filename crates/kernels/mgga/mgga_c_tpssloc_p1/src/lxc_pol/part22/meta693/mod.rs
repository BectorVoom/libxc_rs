//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2273;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta693<F: Float>(t11570: F, t17691: F, t15372: F, t4889: F, t11529: F, t1174: F, t6126: F, t44571: F, t6119: F, t17686: F, t44607: F, t15382: F, t3447: F, t52059: F) -> (F, F, F, F, F, F) {
        let (t65087, t65093, t65112, t65126, t65128, t65136) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2273::<F>(t11570, t17691, t15372, t4889, t11529, t1174, t6126, t44571, t6119, t17686, t44607, t15382, t3447, t52059);
    (t65087, t65093, t65112, t65126, t65128, t65136)
}
