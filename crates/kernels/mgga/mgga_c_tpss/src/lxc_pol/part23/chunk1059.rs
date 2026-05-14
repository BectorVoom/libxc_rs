//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1059/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1059<F: Float>(t1025: F, t11954: F, t11942: F, t1032: F, t11878: F, t9185: F, t141: F, t11932: F, t11938: F, t11952: F, t9221: F, t9223: F, t9226: F, t9228: F, t11845: F, t11848: F, t11850: F, t11853: F, t11892: F, t11896: F, t11899: F, t11904: F, t11908: F, t11911: F, t11913: F, t11916: F, t11919: F, t11922: F, t11925: F, t9183: F, t9192: F, t9194: F, t9196: F, t9297: F, t9306: F) -> (F, F, F, F) {
    let t11955 = t1025 * t11954;
    let t11958 = 0.19931111111111111111e0 * t11942;
    let t11960 = t1032 * t11954;
    let t11962 = t9185 * t11878;
    let t11963 = t141 * t11962;
    let t11965 = 0.26574814814814814816e0 * t9221 + 0.66437037037037037038e-1 * t9223 - 0.19931111111111111111e0 * t9226 - 0.99655555555555555557e-1 * t9228 + 0.36514074074074074074e-1 * t11932 + 0.1898925e1 * t11955 + 0.13287407407407407408e0 * t11938 - t11958 + 0.29896666666666666667e0 * t11952 + 0.3071625e0 * t11960 + 0.36514074074074074075e-1 * t11963;
    let t11967 = -t9297 + 0.18257037037037037037e-1 * t9183 + 0.18257037037037037037e0 * t9192 - 0.54771111111111111111e-1 * t9194 - 0.10954222222222222222e0 * t9196 - t11845 + 0.82156666666666666667e-1 * t11848 + 0.91285185185185185185e-1 * t11850 - t9306 + 0.142419375e1 * t11853 + t11892 - 0.19931111111111111111e0 * t11896 + 0.17938e1 * t11899 + 0.11958666666666666667e1 * t11904 + 0.59793333333333333334e0 * t11908 - t11911 - 0.54771111111111111112e-1 * t11913 - 0.27385555555555555556e-1 * t11916 - 0.16431333333333333333e0 * t11919 + 0.32862666666666666666e0 * t11922 + 0.16431333333333333333e0 * t11925 + t11965;
    (t11955, t11960, t11963, t11967)
}
