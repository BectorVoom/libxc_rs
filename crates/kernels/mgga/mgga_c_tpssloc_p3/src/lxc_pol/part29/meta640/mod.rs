//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta640 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2105;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2106;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2107;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta640<F: Float>(t22986: F, t22996: F, t25249: F, t2633: F, t81602: F, t252: F, t4119: F, t6646: F, t829: F, t25160: F, t814: F, t22690: F, t7520: F, t81573: F, t2627: F, t7510: F, t13171: F, t1510: F, t6657: F, t812: F, t81599: F, t81600: F, t81718: F, t87097: F, t87101: F, t87104: F, t87109: F, t87114: F, t87117: F, t87119: F, t2684: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t13384: F, t2647: F, t22893: F, t23164: F, t25306: F, t81615: F, t25236: F, t13381: F, t1888: F, t7524: F, t81612: F, t81613: F, t4240: F, t81865: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87124, t87127, t87130, t87133, t87135, t87140) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2105::<F>(t22986, t22996, t25249, t2633, t81602, t252, t4119, t6646, t829, t25160, t814, t22690, t7520, t81573);
        let t87146 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2106::<F>(t2627, t7510, t13171, t1510, t2633, t6657, t812, t81599, t81600, t81718, t829, t87097, t87101, t87104, t87109, t87114, t87117, t87119, t87124, t87127, t87133, t87135, t87140);
        let (t87150, t87154, t87155, t87159, t87165) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2107::<F>(t22986, t25249, t2684, t6646, t25324, t6562, t794, t23030, t25258, t13384, t2647, t22893, t23164, t25306);
        let (t87166, t87167, t87171, t87174, t87177, t87183) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2108::<F>(t87165, t81615, t22986, t25236, t2647, t6646, t13381, t1888, t7524, t81612, t81613, t4240, t81865);
    (t87130, t87146, t87150, t87154, t87155, t87159, t87166, t87167, t87171, t87174, t87177, t87183)
}
