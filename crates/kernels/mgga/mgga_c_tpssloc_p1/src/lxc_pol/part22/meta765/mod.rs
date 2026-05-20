//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2584;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2585;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2586;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta765<F: Float>(t1653: F, t5011: F, t19080: F, t4997: F, t1215: F, t5398: F, t11668: F, t11678: F, t11692: F, t15569: F, t15594: F, t15659: F, t1735: F, t18236: F, t18395: F, t19016: F, t22185: F, t27524: F, t3490: F, t3577: F, t3578: F, t45119: F, t4723: F, t4729: F, t475: F, t52813: F, t5971: F, t6203: F, t6230: F, t6232: F, t65424: F, t65444: F, t66388: F, t19047: F, t19040: F, t5005: F, t71095: F, t71097: F, t71106: F, t71109: F, t71112: F, t71114: F, t71118: F, t71217: F, t71221: F, t71225: F, t71227: F, t71230: F, t71233: F, t71236: F, t71238: F, t71241: F, t71245: F, t71247: F, t71249: F, t71251: F, t71255: F, t71313: F, t71315: F, t71317: F, t71319: F, t71543: F, t71545: F, t71547: F, t71655: F, t71657: F, t72045: F, t72047: F, t72050: F, t72052: F, t72058: F, t72061: F, t72065: F, t72067: F, t72071: F, t72073: F, t71697: F, t71700: F, t71704: F, t71707: F, t71711: F, t71784: F, t71786: F, t71788: F, t71790: F, t71793: F, t71795: F, t71797: F, t71800: F, t71803: F, t71806: F, t71809: F, t71811: F, t71814: F, t71817: F, t71819: F) -> (F, F, F, F, F, F, F) {
        let (t72146, t72180) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2584::<F>(t1653, t5011, t19080, t4997, t1215, t5398, t11668, t11678, t11692, t15569, t15594, t15659, t1735, t18236, t18395, t19016, t22185, t27524, t3490, t3577, t3578, t45119, t4723, t4729, t475, t52813, t5971, t6203, t6230, t6232, t65424, t65444, t66388);
        let (t72181, t72183, t72195) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2585::<F>(t19047, t4997, t19040, t5005, t71095, t71097, t71106, t71109, t71112, t71114, t71118, t71217, t71221, t71225, t71227, t71230, t71233, t71236, t71238, t71241, t71245, t71247, t71249, t71251);
        let t72196 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2586::<F>(t71255, t71313, t71315, t71317, t71319, t71543, t71545, t71547, t71655, t71657, t72045, t72047, t72050, t72052, t72058, t72061, t72065, t72067, t72071, t72073);
        let t72198 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2587::<F>(t71697, t71700, t71704, t71707, t71711, t71784, t71786, t71788, t71790, t71793, t71795, t71797, t71800, t71803, t71806, t71809, t71811, t71814, t71817, t71819);
    (t72146, t72180, t72181, t72183, t72195, t72196, t72198)
}
