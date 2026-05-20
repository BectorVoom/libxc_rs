//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta411<F: Float>(t1155: F, t6085: F, t3403: F, t6084: F, t4857: F, t4861: F, t11285: F, t6068: F, t11310: F, t11365: F, t15126: F, t15136: F, t15146: F, t15207: F, t18247: F, t18603: F, t18606: F, t18609: F, t3376: F, t3401: F, t4802: F, t4824: F, t4840: F, t4862: F) -> (F, F, F, F, F, F, F) {
        let (t18612, t18615, t18616, t18619, t18622, t18623, t18630) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1712::<F>(t1155, t6085, t3403, t6084, t4857, t4861, t11285, t6068, t11310, t11365, t15126, t15136, t15146, t15207, t18247, t18603, t18606, t18609, t3376, t3401, t4802, t4824, t4840, t4862);
    (t18612, t18615, t18616, t18619, t18622, t18623, t18630)
}
