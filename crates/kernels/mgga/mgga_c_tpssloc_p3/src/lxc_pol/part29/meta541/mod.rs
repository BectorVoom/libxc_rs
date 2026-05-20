//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1933;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1934;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta541<F: Float>(t26431: F, t26470: F, t1378: F, t7696: F, t794: F, t6897: F, t225: F, t7704: F, t1385: F, t7749: F, t3887: F, t1375: F, t1386: F, t16022: F, t16030: F, t1843: F, t2016: F, t22670: F, t22676: F, t26366: F, t26371: F, t3758: F, t3882: F, t5326: F, t6958: F, t7750: F, t16439: F, t22656: F, t22907: F, t22909: F, t22921: F, t22924: F, t22926: F, t22928: F, t22940: F, t5215: F, t5321: F, t5354: F, t6963: F, t6993: F, t7729: F, t26223: F, t26364: F, t533: F, t1390: F, t1983: F, t16521: F, t1873: F, t16524: F, t7015: F, t5371: F, t6534: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26471, t26472, t26474, t26477, t26482, t26485) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1933::<F>(t26431, t26470, t1378, t7696, t794, t6897, t225, t7704, t1385, t7749, t3887, t1375, t1386, t16022, t16030, t1843, t2016, t22670, t22676, t26366, t26371, t3758, t3882, t5326, t6958, t7750);
        let t26500 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1934::<F>(t16439, t1843, t2016, t22656, t22907, t22909, t22921, t22924, t22926, t22928, t22940, t3758, t5215, t5321, t5354, t6958, t6963, t6993, t7729);
        let (t26502, t26503, t26504, t26505, t26533, t26535, t26537) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1935::<F>(t26223, t26364, t26485, t26500, t533, t1390, t1983, t16521, t1873, t16524, t7015, t5371, t6534);
    (t26471, t26472, t26474, t26477, t26482, t26502, t26503, t26504, t26505, t26533, t26535, t26537)
}
