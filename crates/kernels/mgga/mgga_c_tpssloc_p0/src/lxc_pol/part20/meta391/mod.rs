//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1770;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1771;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta391<F: Float>(t13532: F, t2826: F, t136: F, t10216: F, t1409: F, t2244: F, t10304: F, t2775: F, t3966: F, t607: F, t908: F, t2250: F, t4342: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13533, t13534, t13536, t13537) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1770::<F>(t13532, t2826, t136, t10216, t1409, t2244);
        let (t13538, t13539, t13541, t13542) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1771::<F>(t10304, t13537, t136, t2775, t3966, t607);
        let (t13543, t13544, t13546) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1772::<F>(t13542, t908, t136, t2250, t4342);
    (t13533, t13534, t13536, t13537, t13538, t13539, t13541, t13542, t13543, t13544, t13546)
}
