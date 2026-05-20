//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta402 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1531;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1532;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1533;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1534;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta402<F: Float>(t17817: F, t2988: F, t17183: F, t4518: F, t135: F, t5844: F, t973: F, t10295: F, t10296: F, t13642: F, t13921: F, t13922: F, t13923: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t340: F, t343: F, t974: F, t5838: F, t17801: F, t17805: F, t17809: F, t17811: F, t17814: F, t2960: F, t2986: F, t5839: F, t5845: F, t17157: F, t4510: F, t17161: F, t13798: F, t17152: F, t10236: F, t5392: F, t10235: F, t13851: F, t4514: F, t10287: F, t10333: F, t10339: F, t13893: F, t13896: F, t13907: F, t13909: F, t13915: F, t17766: F, t17798: F, t225: F, t68: F, t369: F, t10457: F, t248: F, t5677: F, t1041: F, t1044: F, t17187: F, t14084: F, t14085: F, t14117: F, t14508: F, t14511: F, t1622: F, t17734: F, t17738: F, t3048: F, t3117: F, t3130: F, t378: F, t4596: F, t4600: F, t4636: F, t4644: F, t5857: F, t5861: F) -> (F, F, F, F, F, F, F, F) {
        let (t17818, t17821, t17827, t17841) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1531::<F>(t17817, t2988, t17183, t4518, t135, t5844, t973, t10295, t10296, t13642, t13921, t13922, t13923, t17241, t17244, t17247, t17250, t17253, t17256, t17280, t17286, t17288, t17290, t17293);
        let (t17843, t17852) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1532::<F>(t17841, t340, t343, t974, t135, t5838, t973, t17801, t17805, t17809, t17811, t17814, t17818, t17821, t17827, t2960, t2986, t5839, t5845);
        let t17873 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1533::<F>(t17157, t4510, t17161, t13798, t17152, t10236, t5392, t10235, t13851, t4514, t10287, t10333, t10339, t13893, t13896, t13907, t13909, t13915, t2986);
        let (t17875, t17876, t17877, t17878, t17884, t17885, t17890) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1534::<F>(t17766, t17798, t17852, t17873, t225, t68, t369, t10457, t248, t5677, t1041, t1044, t17187);
        let t17900 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1535::<F>(t1041, t14084, t14085, t14117, t14508, t14511, t1622, t17734, t17738, t17878, t17885, t17890, t3048, t3117, t3130, t378, t4596, t4600, t4636, t4644, t5857, t5861, t973);
    (t17841, t17843, t17875, t17876, t17877, t17884, t17890, t17900)
}
