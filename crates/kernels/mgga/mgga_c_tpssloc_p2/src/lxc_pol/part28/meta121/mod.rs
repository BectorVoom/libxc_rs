//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk691;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk692;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk693;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk694;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk695;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk696;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk697;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk698;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta121<F: Float>(t261: F, t193: F, t202: F, t2486: F, t2522: F, t2523: F, t2530: F, t2533: F, t2537: F, t2539: F, t2553: F, t2654: F, t2657: F, t2661: F, t2665: F, t2745: F, t2749: F, t766: F, t776: F, t870: F, t2521: F, t1878: F, t268: F, t271: F, t690: F, t885: F, t1043: F, t154: F, t632: F, t2244: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk691::<F>(t261);
        let t2755 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk692::<F>(t193, t202, t2486, t2522, t2523, t2530, t2533, t2537, t2539, t2553, t2654, t2657, t2661, t2665, t2745, t2749, t2752, t766, t776, t870);
        let t2756 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk693::<F>(t2521, t2755);
        let t2764 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk694::<F>(t1878, t268, t271);
        let (t2765, t2766) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk695::<F>(t2764, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk696::<F>(t1043, t154);
        let (t2769, t2770) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk697::<F>(t632);
        let t2771 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk698::<F>(t2244, t2770);
        let (t2772, t2773) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk699::<F>(t2768, t2771, t123);
    (t2751, t2752, t2756, t2764, t2765, t2766, t2768, t2769, t2770, t2771, t2772, t2773)
}
