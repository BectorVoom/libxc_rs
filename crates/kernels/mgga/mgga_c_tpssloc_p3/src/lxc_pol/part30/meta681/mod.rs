//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta681 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2138;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2139;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2140;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2141;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2142;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2143;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2144;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2145;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2146;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta681<F: Float>(t1351: F, t1992: F, t550: F, t6434: F, t6976: F, t22704: F, t22705: F, t28167: F, t26331: F, t26421: F, t26446: F, t5187: F, t22897: F, t3792: F, t57607: F, t6378: F, t6990: F, t81039: F, t81047: F, t90845: F, t90860: F, t90865: F, t90867: F, t93538: F, t96962: F, t96967: F, t96972: F, t96976: F, t96979: F, t19745: F, t81027: F, t12369: F, t19743: F, t22633: F, t562: F, t6330: F, t1307: F, t90591: F, t20018: F, t57499: F, t28163: F, t57618: F, t1332: F, t19805: F, t2013: F, t28156: F, t81061: F, t81066: F, t81073: F, t81075: F, t81076: F, t90899: F, t90913: F, t93563: F, t22881: F, t6347: F, t6637: F, t6888: F, t19631: F, t6968: F, t28130: F, t81228: F, t19748: F, t28134: F, t80798: F, t1985: F, t1998: F, t20009: F, t214: F, t1352: F, t1799: F, t90809: F, t26395: F, t22892: F, t22893: F, t28148: F, t1336: F, t19732: F, t19815: F, t28178: F, t3777: F, t6987: F, t6988: F, t81080: F, t90957: F, t90962: F, t90964: F, t19761: F, t1825: F, t90754: F, t90818: F, t5287: F, t22751: F, t28149: F, t19740: F, t28139: F, t28159: F, t6897: F, t794: F, t19763: F, t19735: F, t22873: F, t26403: F, t26459: F, t5234: F, t5334: F, t6388: F, t6415: F, t81105: F, t90971: F, t90984: F, t90988: F, t93595: F, t19739: F, t3807: F, t28131: F, t81159: F, t552: F, t96964: F) -> (F, F, F, F, F, F, F, F) {
        let (t96986, t96989, t96993) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2138::<F>(t1351, t1992, t550, t6434, t6976, t22704, t22705, t28167, t26331, t26421, t26446, t5187);
        let t96999 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2139::<F>(t1992, t22897, t3792, t57607, t6378, t6990, t81039, t81047, t90845, t90860, t90865, t90867, t93538, t96962, t96967, t96972, t96976, t96979, t96986, t96989, t96993);
        let (t97002, t97007, t97011, t97014) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2140::<F>(t19745, t1992, t81027, t12369, t19743, t22633, t22897, t562, t6330, t1307, t26446, t90591);
        let t97032 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2141::<F>(t1992, t20018, t6976, t550, t57499, t22704, t22705, t28163, t57618, t1332, t19805, t2013, t28156, t81061, t81066, t81073, t81075, t81076, t90899, t90913, t93563, t97002, t97007, t97014);
        let (t97036, t97040, t97043, t97046) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2142::<F>(t22881, t6347, t6637, t6888, t19631, t6968, t22705, t28130, t81228, t19748, t1992, t22897);
        let (t97049, t97055, t97059, t97063) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2143::<F>(t22704, t28134, t80798, t1985, t1998, t20009, t214, t1352, t26331, t6976, t97011, t1799, t6637, t6888, t90809);
        let t97075 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2144::<F>(t26395, t5187, t6637, t6888, t22892, t22893, t28148, t1336, t19732, t19815, t28178, t3777, t6987, t6988, t81080, t90957, t90962, t90964, t97036, t97040, t97043, t97046, t97049, t97055, t97059, t97063);
        let (t97079, t97083, t97087, t97091, t97095, t97106) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2145::<F>(t19761, t1992, t6976, t1825, t22633, t90754, t90818, t26421, t5287, t22751, t28149, t19740, t22897);
        let t97116 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2146::<F>(t22751, t28139, t28159, t6897, t794, t19763, t1992, t6976, t1336, t19735, t22873, t26403, t26459, t5234, t5334, t6388, t6415, t81105, t90971, t90984, t90988, t93595, t97079, t97083, t97087, t97091, t97095, t97106);
        let (t97119, t97124, t97129, t97135) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2147::<F>(t19739, t22633, t3807, t6976, t28131, t81159, t552, t6434, t1307, t6637, t6888, t26331, t26446, t96964);
    (t96999, t97032, t97075, t97116, t97119, t97124, t97129, t97135)
}
