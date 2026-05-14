//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 760/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk760<F: Float>(t1025: F, t5092: F, t2885: F, t5085: F, t1032: F, t2895: F, t5064: F, t141: F, t1038: F, t5068: F, t5072: F, t2880: F, t2892: F, t4044: F, t4093: F, t5066: F, t5070: F, t5074: F, t5086: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5093 = t1025 * t5092;
    let t5099 = t2885 * t5085;
    let t5101 = t1032 * t5092;
    let t5104 = t2895 * t5064;
    let t5105 = t141 * t5104;
    let t5107 = t1038 * t5068;
    let t5108 = t141 * t5107;
    let t5110 = t1038 * t5072;
    let t5111 = t141 * t5110;
    let t5113 = -0.9494625e0 * t5086 + 0.1898925e1 * t5093 + t2880 - 0.19931111111111111111e0 * t4044 - 0.19931111111111111111e0 * t5066 + 0.59793333333333333334e0 * t5070 + 0.29896666666666666667e0 * t5074 + 0.15358125e0 * t5099 + 0.3071625e0 * t5101 + t2892 - 0.10954222222222222222e0 * t4093 - 0.27385555555555555556e-1 * t5105 + 0.16431333333333333333e0 * t5108 + 0.82156666666666666667e-1 * t5111;
    (t5093, t5099, t5101, t5104, t5105, t5107, t5108, t5110, t5111, t5113)
}
