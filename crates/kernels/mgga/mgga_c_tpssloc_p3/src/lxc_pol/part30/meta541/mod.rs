//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1889;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1890;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta541<F: Float>(t26431: F, t26470: F, t1378: F, t7696: F, t794: F, t6897: F, t225: F, t7704: F, t1385: F, t7749: F, t3887: F, t1375: F, t1386: F, t16022: F, t16030: F, t1843: F, t2016: F, t22670: F, t22676: F, t26366: F, t26371: F, t3758: F, t3882: F, t5326: F, t6958: F, t7750: F, t16439: F, t22656: F, t22907: F, t22909: F, t22921: F, t22924: F, t22926: F, t22928: F, t22940: F, t5215: F, t5321: F, t5354: F, t6963: F, t6993: F, t7729: F, t26223: F, t26364: F, t533: F, t1390: F, t1983: F, t1393: F, t1442: F, t1459: F, t1774: F, t1849: F, t1869: F, t22461: F, t26103: F, t26157: F, t26166: F, t26170: F, t26178: F, t26181: F, t26183: F, t4037: F, t5107: F, t6515: F, t6517: F, t6862: F, t6872: F, t7681: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26471, t26472, t26474, t26475, t26477, t26482, t26485) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1889::<F>(t26431, t26470, t1378, t7696, t794, t6897, t225, t7704, t1385, t7749, t3887, t1375, t1386, t16022, t16030, t1843, t2016, t22670, t22676, t26366, t26371, t3758, t3882, t5326, t6958, t7750);
        let t26500 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1890::<F>(t16439, t1843, t2016, t22656, t22907, t22909, t22921, t22924, t22926, t22928, t22940, t3758, t5215, t5321, t5354, t6958, t6963, t6993, t7729);
        let (t26502, t26503, t26504, t26507) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1891::<F>(t26223, t26364, t26485, t26500, t533, t1390, t1983, t1393, t1442, t1459, t1774, t1849, t1869, t22461, t26103, t26157, t26166, t26170, t26178, t26181, t26183, t4037, t5107, t6515, t6517, t6862, t6872, t7681);
    (t26471, t26472, t26474, t26475, t26477, t26482, t26502, t26503, t26504, t26507)
}
