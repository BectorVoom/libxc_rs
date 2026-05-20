//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta665 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2210;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2211;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2212;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2213;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2214;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2215;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2216;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2217;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta665<F: Float>(t16052: F, t1992: F, t22897: F, t26392: F, t80670: F, t16419: F, t6976: F, t22705: F, t26422: F, t81228: F, t16040: F, t22633: F, t3807: F, t54854: F, t550: F, t26331: F, t26421: F, t26446: F, t3719: F, t22704: F, t26466: F, t81022: F, t90806: F, t90807: F, t90812: F, t90816: F, t90821: F, t90825: F, t90829: F, t90832: F, t26461: F, t26433: F, t6883: F, t22716: F, t7741: F, t1834: F, t3791: F, t81039: F, t54840: F, t54883: F, t81061: F, t3793: F, t16041: F, t5336: F, t80798: F, t22724: F, t26436: F, t81037: F, t81041: F, t81043: F, t81047: F, t81050: F, t81066: F, t1307: F, t1352: F, t16037: F, t26423: F, t81159: F, t215: F, t22839: F, t562: F, t80854: F, t16226: F, t22685: F, t26395: F, t3734: F, t6637: F, t81080: F, t16125: F, t3856: F, t12267: F, t1336: F, t22873: F, t5287: F, t7745: F, t81069: F, t81073: F, t81075: F, t81076: F, t81083: F, t81099: F, t1824: F, t6955: F, t2006: F, t5286: F, t1338: F, t26328: F, t26462: F, t6914: F, t26414: F, t26415: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90835, t90837, t90840, t90845, t90848) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2210::<F>(t16052, t1992, t22897, t26392, t80670, t16419, t6976, t22705, t26422, t81228, t16040, t22633, t3807);
        let t90861 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2211::<F>(t1992, t54854, t550, t6976, t26331, t26421, t26446, t3719, t22704, t22705, t26466, t81022, t90806, t90807, t90812, t90816, t90821, t90825, t90829, t90832, t90835, t90837, t90840, t90845, t90848);
        let (t90865, t90867, t90868, t90870, t90873) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2212::<F>(t22704, t22705, t26461, t26433, t6883, t22716, t7741, t1834, t3791, t1992, t550, t6976);
        let (t90876, t90883, t90887, t90889, t90892, t90895) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2213::<F>(t81039, t1992, t54840, t550, t6976, t54883, t81061, t22633, t22897, t26421, t3793, t16041);
        let t90902 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2214::<F>(t22704, t5336, t80798, t22724, t26436, t81037, t81041, t81043, t81047, t81050, t90865, t90867, t90868, t90873, t90876, t90883, t90887, t90889, t90892, t90895);
        let (t90903, t90907, t90910, t90913, t90914) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2215::<F>(t81066, t1307, t1352, t1834, t22633, t6976, t16037, t1992, t22897, t26423, t81159, t215, t22839);
        let (t90915, t90917, t90921, t90925, t90929) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2216::<F>(t562, t80854, t16226, t90914, t22685, t26395, t3734, t6637, t81080, t16125, t1992, t6976);
        let t90939 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2217::<F>(t22633, t26421, t3856, t6976, t12267, t1336, t22873, t5287, t7745, t81069, t81073, t81075, t81076, t81083, t81099, t90903, t90907, t90910, t90913, t90917, t90921, t90925, t90929);
        let (t90942, t90946, t90952, t90957, t90962, t90963) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2218::<F>(t1824, t6955, t2006, t5286, t1338, t26328, t26462, t6914, t22705, t26414, t81228, t26415, t81159);
    (t90861, t90870, t90902, t90915, t90939, t90942, t90946, t90952, t90957, t90962, t90963)
}
