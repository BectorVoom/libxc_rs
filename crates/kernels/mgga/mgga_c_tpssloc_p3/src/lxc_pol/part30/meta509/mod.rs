//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1830;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta509<F: Float>(t25717: F, t6784: F, t2770: F, t381: F, t3961: F, t25510: F, t23613: F, t7603: F, t1003: F, t1058: F, t23327: F, t23346: F, t23712: F, t25429: F, t25563: F, t25568: F, t25706: F, t25708: F, t25714: F, t3186: F, t353: F, t6680: F, t6687: F, t7604: F, t7615: F, t7622: F, t25482: F, t25527: F, t25560: F, t1055: F, t23384: F, t7566: F, t23394: F, t4664: F, t6704: F, t1634: F, t6815: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25718, t25721) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1829::<F>(t25717, t6784, t2770, t381);
        let (t25722, t25723, t25726, t25729) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1830::<F>(t25721, t3961, t25510, t23613, t7603, t1003, t1058, t23327, t23346, t23712, t25429, t25563, t25568, t25706, t25708, t25714, t25718, t3186, t353, t6680, t6687, t7604, t7615, t7622);
        let (t25731, t25732, t25736, t25738, t25739, t25742) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1831::<F>(t25482, t25527, t25560, t25729, t1055, t23384, t7566, t23394, t4664, t6704, t1634, t6815);
    (t25718, t25721, t25722, t25723, t25726, t25731, t25732, t25736, t25738, t25739, t25742)
}
