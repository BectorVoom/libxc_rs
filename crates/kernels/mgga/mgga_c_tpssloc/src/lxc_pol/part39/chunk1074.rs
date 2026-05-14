//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1074/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1074<F: Float>(t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F, t11195: F, t11204: F, t11211: F, t11213: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t14868: F, t14870: F, t14887: F) -> (F,) {
    let t14890 = 0.21908444444444444444e0 * t14781;
    let t14911 = -0.1898925e1 * t14809 - 0.9494625e0 * t14811 + 0.3071625e0 * t14814 + 0.15358125e0 * t14816 + 0.36514074074074074074e-1 * t14818 + 0.26574814814814814816e0 * t11137 + 0.66437037037037037038e-1 * t11139 - 0.19931111111111111111e0 * t11141 - 0.99655555555555555557e-1 * t11143 + 0.3071625e0 * t14824 + 0.33218518518518518518e0 * t14728;
    let t14913 = -t11195 - t11204 + 0.13287407407407407408e0 * t14702 - t14868 + 0.29896666666666666667e0 * t14708 - t14870 + 0.82156666666666666667e-1 * t14713 + 0.1898925e1 * t14759 + 0.18257037037037037037e0 * t11211 + 0.18257037037037037037e-1 * t11213 + t14887 + 0.36514074074074074075e-1 * t14779 - t14890 - 0.54771111111111111112e-1 * t14784 - 0.27385555555555555556e-1 * t14787 - 0.16431333333333333333e0 * t14790 + 0.32862666666666666666e0 * t14793 + 0.16431333333333333333e0 * t14796 + 0.49293999999999999999e0 * t14799 + 0.142419375e1 * t14802 - 0.76790625e-1 * t14805 + t14911;
    (t14913,)
}
