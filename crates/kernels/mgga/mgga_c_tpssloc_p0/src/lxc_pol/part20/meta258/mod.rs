//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta258 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1391;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1392;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1393;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1394;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta258<F: Float>(t10121: F, t193: F, t202: F, t2379: F, t2522: F, t2523: F, t2553: F, t262: F, t4314: F, t766: F, t776: F, t870: F, t9450: F, t9457: F, t9458: F, t9463: F, t9469: F, t9470: F, t9476: F, t9484: F, t9496: F, t9516: F, t2745: F, t2752: F, t1877: F, t868: F, t9684: F, t9715: F, t9718: F, t9724: F, t9727: F, t9780: F, t9789: F, t9863: F, t9865: F, t9867: F, t9870: F, t2749: F, t261: F, t2751: F, t9793: F, t9797: F, t9820: F, t9824: F, t9872: F, t9876: F, t9881: F, t9884: F, t9887: F, t9890: F, t9894: F, t9896: F, t9853: F, t9859: F, t9900: F, t9903: F, t9907: F, t9911: F, t9914: F, t9917: F, t9921: F, t9923: F, t9925: F, t9928: F, t9931: F, t9934: F) -> (F, F, F, F, F) {
        let t10125 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1391::<F>(t10121, t193, t202, t2379, t2522, t2523, t2553, t262, t4314, t766, t776, t870, t9450, t9457, t9458, t9463, t9469, t9470, t9476, t9484, t9496, t9516);
        let (t10126, t10134, t10138) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1392::<F>(t2745, t870, t2553, t262, t2752, t1877, t2522, t4314, t776, t868, t9684, t9715, t9718, t9724, t9727, t9780, t9789, t9863, t9865, t9867, t9870);
        let (t10140, t10143, t10147) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1393::<F>(t2749, t868, t261, t2751, t193, t202, t9793, t9797, t9820, t9824, t9872, t9876, t9881, t9884, t9887, t9890, t9894, t9896);
        let t10148 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1394::<F>(t9853, t9859, t9900, t9903, t9907, t9911, t9914, t9917, t9921, t9923, t9925, t9928, t9931, t9934);
        let t10150 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1395::<F>(t10125, t10138, t10147, t10148);
    (t10126, t10134, t10140, t10143, t10150)
}
