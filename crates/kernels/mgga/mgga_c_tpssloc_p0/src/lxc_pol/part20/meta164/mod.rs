//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1035;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1036;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1037;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1038;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta164<F: Float>(t1315: F, t1341: F, t1354: F, t1363: F, t1369: F, t3733: F, t3762: F, t3763: F, t3766: F, t3770: F, t3774: F, t3778: F, t3781: F, t3783: F, t3790: F, t3795: F, t3800: F, t3803: F, t3809: F, t3853: F, t3858: F, t3864: F, t3867: F, t3872: F, t3876: F, t559: F, t539: F, t1373: F, t225: F, t1376: F, t566: F, t68: F, t1385: F, t3787: F, t562: F, t3793: F, t1338: F, t1372: F) -> (F, F, F, F, F, F, F, F, F) {
        let t3879 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1035::<F>(t1315, t1341, t1354, t1363, t1369, t3733, t3762, t3763, t3766, t3770, t3774, t3778, t3781, t3783, t3790, t3795, t3800, t3803, t3809, t3853, t3858, t3864, t3867, t3872, t3876, t559);
        let (t3880, t3882) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1036::<F>(t3879, t539, t1373, t225);
        let t3887 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1037::<F>(t1376, t566, t68);
        let (t3888, t3889) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1038::<F>(t1385, t3887);
        let (t3897, t3898, t3901) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1039::<F>(t3787, t562, t3793, t1338, t1372);
    (t3879, t3880, t3882, t3887, t3888, t3889, t3897, t3898, t3901)
}
