//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk845;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk846;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk847;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk848;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk849;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk850;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk851;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta160<F: Float>(t3477: F, t974: F, t1174: F, t3430: F, t3433: F, t3436: F, t3443: F, t3447: F, t3452: F, t3457: F, t3461: F, t3472: F, t491: F, t1190: F, t1235: F, t1191: F, t225: F, t1202: F, t1226: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t3408: F, t3410: F, t3413: F, t3417: F, t3421: F, t3425: F, t475: F, t1214: F, t248: F, t3030: F, t466: F, t3032: F, t1208: F, t476: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3478, t3481) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk845::<F>(t3477, t974, t1174, t3430, t3433, t3436, t3443, t3447, t3452, t3457, t3461, t3472);
        let (t3482, t3484, t3487) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk846::<F>(t3481, t491, t1190, t1235, t1191, t225);
        let t3490 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk847::<F>(t1202, t1226);
        let t3493 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk848::<F>(t3258, t3261, t3268, t3310, t3318, t3408, t3410, t3413, t3417, t3421, t3425);
        let t3494 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk849::<F>(t3493, t475);
        let t3496 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk850::<F>(t1214, t248, t3494);
        let (t3499, t3500) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk851::<F>(t3030, t466, t3032);
        let t3502 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk852::<F>(t1208, t476);
    (t3478, t3481, t3482, t3484, t3487, t3490, t3493, t3494, t3496, t3499, t3500, t3502)
}
