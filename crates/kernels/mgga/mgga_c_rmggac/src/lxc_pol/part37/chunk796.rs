//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 796/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk796<F: Float>(t15621: F, t7487: F, t74013: F, t74018: F, t74021: F, t74024: F, t74027: F, t74033: F, t74036: F, t74043: F, t74046: F, t70806: F, t70809: F, t70812: F, t15492: F, t2160: F, t638: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t76819 = t7487 * t15621;
    let t76820 = 0.96056421943322389208e-3 * t76819;
    let t76821 = 0.1276937996798935182e-4 * t74013;
    let t76823 = 0.1276937996798935182e-4 * t74018;
    let t76828 = 0.2553875993597870364e-4 * t74021;
    let t76829 = 0.3830813990396805546e-4 * t74024;
    let t76830 = 0.2553875993597870364e-4 * t74027;
    let t76831 = 0.1276937996798935182e-4 * t74033;
    let t76832 = 0.15961724959986689775e-4 * t74036;
    let t76834 = 0.15961724959986689775e-4 * t74043;
    let t76835 = 0.15961724959986689775e-4 * t74046;
    let t76836 = 0.79828278012425390426e-1 * t70806;
    let t76837 = 0.11974241701863808564e0 * t70809;
    let t76838 = 0.79828278012425390426e-1 * t70812;
    let t76840 = t638 * t2160 * t15492;
    (t76820, t76821, t76823, t76828, t76829, t76830, t76831, t76832, t76834, t76835, t76836, t76837, t76838, t76840)
}
