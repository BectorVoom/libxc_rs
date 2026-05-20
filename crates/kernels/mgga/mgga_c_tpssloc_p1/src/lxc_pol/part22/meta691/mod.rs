//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta691 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2270;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta691<F: Float>(t11583: F, t17691: F, t3428: F, t6109: F, t1174: F, t6146: F, t698: F, t6140: F, t18321: F, t3435: F, t15281: F, t18563: F, t3432: F, t11529: F, t6130: F, t15282: F, t4889: F, t18558: F, t3431: F, t11570: F, t17686: F, t15299: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t64870, t64878, t64881, t64885, t64951, t64969) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2270::<F>(t11583, t17691, t3428, t6109, t1174, t6146, t698, t6140, t18321, t3435, t15281, t18563);
        let (t64976, t64979, t64981, t64988, t64994, t65002) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2271::<F>(t18321, t3432, t11529, t1174, t6130, t15282, t4889, t18558, t3431, t11570, t17686, t15299);
    (t64870, t64878, t64881, t64885, t64951, t64969, t64976, t64979, t64981, t64988, t64994, t65002)
}
