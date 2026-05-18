//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 901/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk901<F: Float>(t735: F, t8017: F, t2214: F, t7813: F, t7857: F, t2332: F, t692: F, t2210: F, t720: F, t177: F, t2240: F, t737: F) -> (F, F, F, F, F, F, F) {
    let t8019 = F::new(0.5848223622634646207e0) * t735 * t8017;
    let t8021 = t7857 * t7813 * t2214;
    let t8023 = F::new(0.10389515463408878255e3) * t735 * t8021;
    let t8024 = t692 * t2332;
    let t8027 = t2210 * t7813 * t720;
    let t8029 = F::new(0.35089341735807877242e1) * t735 * t8027;
    let t8034 = t2240 * t177;
    let t8035 = t8034 * t737;
    (t8019, t8021, t8023, t8024, t8027, t8029, t8035)
}
