//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk986;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk987;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk988;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk989;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk990;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk991;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta286<F: Float>(t21013: F, t218: F, t1528: F, t17052: F, t17090: F, t17092: F, t21034: F, t21036: F, t21038: F, t21050: F, t21054: F, t21061: F, t259: F, t4147: F, t4268: F, t5637: F, t5658: F, t855: F, t10143: F, t1484: F, t16625: F, t193: F, t202: F, t20777: F, t20778: F, t20800: F, t20815: F, t2522: F, t4310: F, t5544: F, t766: F, t870: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t20818: F, t20820: F, t20822: F, t20823: F, t20824: F, t20827: F, t20829: F, t20830: F, t20831: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F, t20752: F, t20772: F, t1580: F, t5774: F, t2929: F, t951: F, t959: F, t10523: F, t2932: F, t1589: F, t17934: F, t10629: F, t10632: F, t4483: F, t5808: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21064, t21066) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk986::<F>(t21013, t218, t1528, t17052, t17090, t17092, t21034, t21036, t21038, t21050, t21054, t21061, t259, t4147, t4268, t5637, t5658, t855);
        let t21073 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk987::<F>(t10143, t1484, t16625, t193, t202, t20777, t20778, t20800, t20815, t21066, t2522, t4310, t5544, t766, t870, t9820, t9824, t9876, t9884, t9887, t9890);
        let t21074 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk988::<F>(t20818, t20820, t20822, t20823, t20824, t20827, t20829, t20830, t20831, t9853, t9859, t9894, t9907, t9921);
        let t21076 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk989::<F>(t20752, t20772, t21073, t21074);
        let t21089 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk990::<F>(t1580, t5774);
        let (t21091, t21093, t21095, t21097, t21099, t21101, t21103, t21105) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk991::<F>(t21089, t2929, t951, t959, t10523, t2932, t1589, t17934, t10629, t10632, t4483, t5808);
    (t21064, t21066, t21076, t21089, t21091, t21093, t21095, t21097, t21099, t21101, t21103, t21105)
}
