//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2148;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2149;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2150;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta682<F: Float>(t28164: F, t6914: F, t22704: F, t22705: F, t28181: F, t19889: F, t91004: F, t91006: F, t28182: F, t19660: F, t22633: F, t3807: F, t6976: F, t1336: F, t22873: F, t28171: F, t28174: F, t3777: F, t5230: F, t6420: F, t7747: F, t91002: F, t91011: F, t93605: F, t93615: F, t97119: F, t97124: F, t97129: F, t97135: F, t22685: F, t22881: F, t6330: F, t6637: F, t22893: F, t28142: F, t80681: F, t2006: F, t6387: F, t28143: F, t80727: F, t6414: F, t1824: F, t7722: F, t1338: F, t28107: F, t1352: F, t16047: F, t1814: F, t1825: F, t19654: F, t19744: F, t26401: F, t26403: F, t26453: F, t5250: F, t5287: F, t5334: F, t5344: F, t81147: F, t81149: F, t81154: F, t81187: F, t81197: F, t90952: F) -> (F, F, F, F, F) {
        let (t97137, t97142, t97146, t97148, t97152) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2148::<F>(t28164, t6914, t22704, t22705, t28181, t19889, t91004, t91006, t28182, t19660, t22633, t3807, t6976);
        let t97154 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2149::<F>(t1336, t22873, t28171, t28174, t3777, t5230, t6420, t7747, t91002, t91011, t93605, t93615, t97119, t97124, t97129, t97135, t97137, t97142, t97146, t97148, t97152);
        let (t97158, t97161, t97172, t97179, t97181) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2150::<F>(t22685, t22881, t6330, t6637, t22893, t28142, t80681, t2006, t6387, t28143, t80727, t6414);
        let (t97189, t97196) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2151::<F>(t1824, t7722, t1338, t28107, t1336, t1352, t16047, t1814, t1825, t19654, t19744, t26401, t26403, t26453, t5250, t5287, t5334, t5344, t81147, t81149, t81154, t81187, t81197, t90952, t97158, t97161, t97172, t97179, t97181);
    (t97154, t97172, t97181, t97189, t97196)
}
