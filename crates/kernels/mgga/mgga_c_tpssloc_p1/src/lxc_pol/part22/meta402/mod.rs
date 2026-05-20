//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1697;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1698;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta402<F: Float>(t18416: F, t3451: F, t3448: F, t6144: F, t18225: F, t4908: F, t11583: F, t5392: F, t3449: F, t18221: F, t15320: F, t4904: F, t15313: F, t4919: F, t11531: F, t15265: F, t15376: F, t18404: F, t18410: F, t18413: F, t3447: F, t4901: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18417, t18420) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1697::<F>(t18416, t3451, t3448, t6144);
        let (t18421, t18424, t18427) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1698::<F>(t18420, t3451, t18225, t4908, t11583, t5392);
        let (t18428, t18431, t18434, t18437, t18442) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1699::<F>(t18427, t3449, t18221, t4908, t15320, t4904, t15313, t4919, t11531, t15265, t15376, t18404, t18410, t18413, t18417, t18421, t18424, t3447, t4901);
    (t18417, t18420, t18421, t18424, t18427, t18428, t18431, t18434, t18437, t18442)
}
