//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1801;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1802;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta447<F: Float>(t1343: F, t19732: F, t820: F, t120: F, t6387: F, t5248: F, t5250: F, t5234: F, t5245: F, t12283: F, t6396: F, t3805: F, t3807: F, t16306: F, t6394: F, t16305: F, t16225: F, t16311: F, t1825: F, t5308: F, t16224: F, t12286: F, t1341: F, t16239: F, t16241: F, t16269: F, t16290: F, t16294: F, t16317: F, t16325: F, t16331: F, t16338: F, t16341: F, t3778: F, t3803: F, t5246: F, t5252: F, t6390: F, t6417: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19868, t19871) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1801::<F>(t1343, t19732, t820, t120, t6387);
        let (t19873, t19876, t19879, t19882, t19886) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1802::<F>(t19871, t5248, t5250, t5234, t5245, t12283, t6396, t3805, t3807, t16306, t6394, t16305);
        let (t19890, t19894, t19899) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1803::<F>(t16225, t16311, t16305, t1825, t5308, t16224, t12286, t1341, t16239, t16241, t16269, t16290, t16294, t16317, t16325, t16331, t16338, t16341, t19868, t19873, t19876, t19879, t19882, t19886, t3778, t3803, t5246, t5252, t6390, t6417);
    (t19868, t19871, t19873, t19876, t19879, t19882, t19886, t19890, t19894, t19899)
}
