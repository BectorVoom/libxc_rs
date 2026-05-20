//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta521 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1731;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1732;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta521<F: Float>(t29274: F, t29285: F, t539: F, t1807: F, t7918: F, t2085: F, t6361: F, t12021: F, t2091: F, t6439: F, t1842: F, t7936: F, t3887: F, t1375: F, t1843: F, t24071: F, t26184: F, t26198: F, t26200: F, t26345: F, t27009: F, t27068: F, t28118: F, t28193: F, t28196: F, t28201: F, t5321: F, t568: F, t7925: F, t553: F, t24127: F, t6388: F, t1336: F, t1814: F, t2089: F, t24099: F, t26381: F, t26393: F, t26406: F, t28132: F, t28136: F, t28140: F, t28144: F, t28150: F, t544: F, t6378: F, t7934: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29286, t29287, t29290, t29293, t29299, t29310) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1731::<F>(t29274, t29285, t539, t1807, t7918, t2085, t6361, t12021, t2091, t6439, t1842, t7936);
        let (t29311, t29314) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1732::<F>(t29310, t3887, t1375, t1843, t24071, t26184, t26198, t26200, t26345, t27009, t27068, t28118, t28193, t28196, t28201, t29287, t29290, t29293, t29299, t5321, t568, t7925);
        let (t29327, t29339, t29342) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1733::<F>(t29286, t553, t24127, t6388, t1336, t1814, t2089, t24099, t26381, t26393, t26406, t28132, t28136, t28140, t28144, t28150, t544, t6378, t7934);
    (t29286, t29287, t29290, t29293, t29299, t29311, t29314, t29327, t29339, t29342)
}
