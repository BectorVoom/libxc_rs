//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1736;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1737;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta502<F: Float>(t5: F, t26938: F, t26964: F, t112: F, t24990: F, t7170: F, t24432: F, t25988: F, t2035: F, t671: F, t1393: F, t1459: F, t1849: F, t1983: F, t2040: F, t2079: F, t22574: F, t26114: F, t26898: F, t26902: F, t26906: F, t4037: F, t510: F, t5361: F, t650: F, t6876: F, t7042: F, t7166: F, t7218: F, t7685: F, t7890: F, t7900: F, t7941: F, t26198: F, t12020: F, t2091: F, t5325: F, t26200: F, t3887: F, t5353: F, t1375: F, t26184: F, t26187: F, t26191: F, t26195: F, t26204: F, t26207: F, t26212: F, t26224: F, t3758: F, t5326: F, t7194: F, t7925: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26966, t26967, t26969, t26974, t26977) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1736::<F>(t5, t26938, t26964, t112, t24990, t7170, t24432, t25988, t2035, t671);
        let t26982 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1737::<F>(t1393, t1459, t1849, t1983, t2040, t2079, t22574, t26114, t26898, t26902, t26906, t26967, t26969, t26974, t26977, t4037, t510, t5361, t650, t6876, t7042, t7166, t7218, t7685, t7890, t7900, t7941);
        let (t26989, t26990, t26996, t27005) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1738::<F>(t26198, t12020, t2091, t5325, t26200, t3887, t5353, t1375, t26184, t26187, t26191, t26195, t26204, t26207, t26212, t26224, t3758, t5326, t7194, t7925);
    (t26966, t26967, t26969, t26974, t26977, t26982, t26989, t26990, t26996, t27005)
}
