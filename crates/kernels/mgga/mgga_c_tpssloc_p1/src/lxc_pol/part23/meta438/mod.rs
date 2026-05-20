//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta438 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1281;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta438<F: Float>(t15338: F, t18409: F, t3447: F, t20217: F, t3450: F, t18469: F, t52059: F, t4904: F, t64763: F, t18532: F, t4889: F, t1174: F, t135: F, t22040: F, t18321: F, t4916: F, t11583: F, t21510: F, t11570: F, t15419: F, t21745: F, t20234: F, t44505: F, t1171: F, t22104: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t73395, t73405, t73417, t73420, t73424, t73427) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1281::<F>(t15338, t18409, t3447, t20217, t3450, t18469, t52059, t4904, t64763, t18532, t4889, t1174, t135, t22040);
        let (t73433, t73444, t73451, t73491, t73496, t73523) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1282::<F>(t18321, t4916, t11583, t21510, t11570, t15419, t21745, t3447, t20234, t44505, t1171, t22104);
    (t73395, t73405, t73417, t73420, t73424, t73427, t73433, t73444, t73451, t73491, t73496, t73523)
}
