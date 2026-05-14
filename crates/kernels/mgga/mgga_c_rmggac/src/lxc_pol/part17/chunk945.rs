//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 945/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk945<F: Float>(t47813: F, t7720: F, t40001: F, t9222: F, t10199: F, t275: F, t42085: F, t8443: F, t118: F, t1986: F, t571: F, t615: F, t7717: F, t1707: F, t2124: F, t42024: F, t42027: F, t42035: F, t42042: F, t42044: F, t43990: F, t47795: F, t47797: F, t47800: F, t47802: F, t47804: F, t47809: F, t903: F) -> (F,) {
    let t47814 = t7720 * t47813;
    let t47816 = t9222 * t40001;
    let t47818 = t275 * t10199;
    let t47821 = t42085 * t8443;
    let t47825 = t1986 * t118 * t571 * t615;
    let t47826 = t7717 * t47825;
    let t47828 = 0.35922725105591425692e0 * t903 * t2124 * t1707 - t42024 - t42027 - 0.5987120850931904282e-1 * t47795 - t42035 + 0.8980681276397856423e-1 * t47797 + 0.60975299583150056628e-3 * t42042 - 0.20455996240684006296e-1 * t47800 - 0.81823984962736025184e-1 * t47802 - 0.20455996240684006296e-1 * t47804 - 0.17025839957319135759e-4 * t47809 + 0.85129199786595678796e-5 * t47814 + 0.1064114997332445985e-4 * t47816 + 2.0 * t47818 + 0.59590439850616975158e-4 * t42044 + 0.19863479950205658386e-4 * t47821 + 0.1064114997332445985e-4 * t47826 + t43990;
    (t47828,)
}
