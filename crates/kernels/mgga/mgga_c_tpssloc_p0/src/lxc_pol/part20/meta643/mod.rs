//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2355;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2356;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta643<F: Float>(t13823: F, t2960: F, t13816: F, t2970: F, t973: F, t13828: F, t10224: F, t4522: F, t13895: F, t1599: F, t2402: F, t13908: F, t10263: F, t4528: F, t12606: F, t2989: F, t10241: F, t13861: F, t1600: F, t2986: F, t2988: F, t3008: F, t3014: F, t343: F, t42554: F, t43061: F, t4514: F, t4540: F, t4543: F, t4546: F, t344: F, t43052: F, t4343: F, t2978: F, t4338: F, t697: F, t43053: F, t13542: F, t13779: F, t13546: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48297, t48302, t48317, t48321, t48329, t48336, t48338) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2355::<F>(t13823, t2960, t13816, t2970, t973, t13828, t10224, t4522, t13895, t1599, t2402, t13908);
        let t48361 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2356::<F>(t48338, t10263, t4528, t12606, t2989, t10241, t13861, t1600, t2986, t2988, t3008, t3014, t343, t42554, t43061, t4514, t4540, t4543, t4546, t48329, t48336, t973);
        let (t48374, t48379, t48382, t48384, t48387) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2357::<F>(t2986, t344, t43052, t4343, t2978, t4338, t697, t43053, t4514, t13542, t13779, t13546);
    (t48297, t48302, t48317, t48321, t48361, t48374, t48379, t48382, t48384, t48387)
}
