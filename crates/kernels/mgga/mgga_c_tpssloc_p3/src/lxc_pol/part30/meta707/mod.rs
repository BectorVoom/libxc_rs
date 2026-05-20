//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2332;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2333;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2334;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta707<F: Float>(t100822: F, t100864: F, t96749: F, t96793: F, t96840: F, t97814: F, t97859: F, t97906: F, t16524: F, t26545: F, t1873: F, t66958: F, t55388: F, t7015: F, t20173: F, t28896: F, t28893: F, t6534: F, t1401: F, t96729: F, t26542: F, t1458: F, t26135: F, t3941: F, t4072: F, t7467: F, t28017: F, t3938: F, t12524: F, t28899: F, t20176: F, t23877: F, t23880: F, t26523: F, t5456: F, t5493: F, t577: F, t83980: F, t96351: F, t75795: F, t7769: F, t5371: F, t112: F, t28868: F, t26550: F, t55353: F, t16521: F, t19534: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t100867, t100871, t100873) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2332::<F>(t100822, t100864, t96749, t96793, t96840, t97814, t97859, t97906, t16524, t26545, t1873, t66958);
        let (t100875, t100879, t100883, t100885, t100887, t100890) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2333::<F>(t55388, t7015, t20173, t28896, t28893, t6534, t1401, t96729, t16524, t26542, t1458, t26135, t3941);
        let t100900 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2334::<F>(t3941, t4072, t7467, t28017, t3938, t12524, t28899, t100867, t100871, t100873, t100875, t100879, t100883, t100885, t100887, t100890, t20176, t23877, t23880, t26523, t5456, t5493, t577, t83980, t96351);
        let (t100902, t100908, t100911, t100915, t100917, t100921, t100924) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2335::<F>(t75795, t7769, t26135, t5371, t112, t28868, t16524, t26550, t55353, t16521, t7467, t1873, t19534, t3941);
    (t100867, t100900, t100902, t100908, t100911, t100915, t100917, t100921, t100924)
}
