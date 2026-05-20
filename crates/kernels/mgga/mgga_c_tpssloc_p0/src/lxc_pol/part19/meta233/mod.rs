//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk940;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk941;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk942;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk943;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk944;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk945;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta233<F: Float>(t1060: F, t11077: F, t11023: F, t3201: F, t1003: F, t10359: F, t1058: F, t1061: F, t1063: F, t11024: F, t11028: F, t11031: F, t11034: F, t11037: F, t11040: F, t11043: F, t11046: F, t11049: F, t11051: F, t11055: F, t11059: F, t11061: F, t11065: F, t11067: F, t3076: F, t3180: F, t3186: F, t3189: F, t3193: F, t3197: F, t3200: F, t3202: F, t3204: F, t353: F, t384: F, t1055: F, t10160: F, t10167: F, t10170: F, t10182: F, t1052: F, t1066: F, t11008: F, t11010: F, t11013: F, t11016: F, t11018: F, t3026: F, t3169: F, t3176: F, t3207: F, t388: F, t1068: F, t3213: F, t3215: F, t390: F, t10521: F, t10528: F, t10607: F, t10625: F, t10627: F, t10635: F, t1070: F, t10711: F, t10729: F, t10733: F, t10849: F, t10851: F, t193: F, t336: F, t10622: F, t10649: F, t10652: F, t10654: F, t10657: F, t10665: F, t10699: F, t10707: F, t10715: F, t10739: F, t10819: F, t10855: F, t3209: F, t3216: F, t4700: F, t25: F, t265: F, t394: F, t10150: F, t1074: F, t2249: F, t2250: F, t2756: F, t3220: F, t396: F, t40: F, t606: F, t607: F, t873: F, t9257: F, t9258: F, dens_threshold: F, rho0: F, zeta_threshold: F, t300: F, t3368: F, t1166: F, t1155: F, t3377: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11078, t11081, t11084) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk940::<F>(t1060, t11077, t11023, t3201, t1003, t10359, t1058, t1061, t1063, t11024, t11028, t11031, t11034, t11037, t11040, t11043, t11046, t11049, t11051, t11055, t11059, t11061, t11065, t11067, t3076, t3180, t3186, t3189, t3193, t3197, t3200, t3202, t3204, t353, t384);
        let (t11085, t11087) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk941::<F>(t1055, t11084, t10160, t10167, t10170, t10182, t1052, t1066, t11008, t11010, t11013, t11016, t11018, t3026, t3169, t3176, t3207, t388);
        let (t11094, t11098) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk942::<F>(t1068, t3213, t3215, t390, t10521, t10528, t10607, t10625, t10627, t10635, t1070, t10711, t10729, t10733, t10849, t10851, t11087, t193, t336);
        let t11103 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk943::<F>(t10622, t10649, t10652, t10654, t10657, t10665, t1068, t10699, t10707, t10715, t10739, t10819, t10855, t3209, t3216, t4700);
        let (t11105, t11115) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk944::<F>(t25, t265, t394, t10150, t11098, t11103, t1074, t2249, t2250, t2756, t3220, t396, t40, t606, t607, t873, t9257, t9258, dens_threshold, rho0, zeta_threshold);
        let t11122 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk945::<F>(t9257);
        let (t11126, t11128, t11129) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk946::<F>(t300, t3368, t1166, t1155, t3377);
    (t11078, t11081, t11084, t11085, t11087, t11094, t11105, t11115, t11122, t11126, t11128, t11129)
}
