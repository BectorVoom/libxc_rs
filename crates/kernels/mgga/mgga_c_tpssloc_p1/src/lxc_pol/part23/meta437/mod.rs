//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1279;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1280;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta437<F: Float>(t11570: F, t20234: F, t18457: F, t4889: F, t18321: F, t4896: F, t18451: F, t1174: F, t22081: F, t44562: F, t22046: F, t3431: F, t15281: F, t22051: F, t11539: F, t22055: F, t18454: F, t22059: F, t18529: F, t135: F, t22034: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t73225, t73272, t73274, t73276, t73279, t73287) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1279::<F>(t11570, t20234, t18457, t4889, t18321, t4896, t18451, t1174, t22081, t44562, t22046, t3431);
        let (t73290, t73307, t73314, t73330, t73386, t73389) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1280::<F>(t1174, t15281, t22051, t11539, t22055, t18454, t4889, t22059, t3431, t18529, t135, t22034);
    (t73225, t73272, t73274, t73276, t73279, t73287, t73290, t73307, t73314, t73330, t73386, t73389)
}
