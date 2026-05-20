//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2055;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta644<F: Float>(t25641: F, t82892: F, t25638: F, t6735: F, t23418: F, t4669: F, t13765: F, t23419: F, t10469: F, t23470: F, t3: F, t82986: F, t23437: F, t4630: F, t82943: F, t1933: F, t1937: F, t3966: F, t25655: F, t82895: F, t25661: F, t1036: F, t25664: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t88488, t88503, t88513, t88517, t88537) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2055::<F>(t25641, t82892, t25638, t6735, t23418, t4669, t13765, t23419, t10469, t23470, t3, t82986);
        let (t88548, t88566, t88569, t88575, t88577, t88582) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2056::<F>(t23437, t4630, t25641, t82943, t1933, t1937, t3966, t25655, t82895, t25661, t1036, t25664);
    (t88488, t88503, t88513, t88517, t88537, t88548, t88566, t88569, t88575, t88577, t88582)
}
