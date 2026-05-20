//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta692<F: Float>(t15285: F, t4889: F, t17686: F, t44505: F, t15363: F, t1174: F, t15281: F, t18549: F, t18554: F, t11570: F, t17635: F, t11583: F) -> (F, F, F, F, F, F, F) {
        let (t65008, t65018, t65023, t65035, t65041, t65056, t65077) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2272::<F>(t15285, t4889, t17686, t44505, t15363, t1174, t15281, t18549, t18554, t11570, t17635, t11583);
    (t65008, t65018, t65023, t65035, t65041, t65056, t65077)
}
