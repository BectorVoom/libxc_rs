//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1075/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1075<F: Float>(t81920: F, t81954: F, t81907: F, t81909: F, t81912: F, t81918: F, t81924: F, t81926: F, t81928: F, t81930: F, t81934: F, t81936: F, t81940: F, t81943: F, t81946: F, t81949: F, t81957: F, t81960: F, t81964: F, t81972: F) -> (F,) {
    let t84921 = 595.0 / 2592.0 * t81920;
    let t84932 = 0.67287926823567318088e-4 * t81954;
    let t84937 = 0.24223653656484234512e-2 * t81907 + 0.84782787797694820791e-2 * t81909 - 0.67826230238155856633e-1 * t81912 - 0.40372756094140390854e-3 * t81918 - t84921 + 0.20186378047070195427e-3 * t81924 - 7.0 / 384.0 * t81926 + 119.0 / 1152.0 * t81928 - t81930 / 24.0 - 0.4069573814289351398e0 * t81934 + 0.50869672678616892474e-1 * t81936 - 0.24223653656484234512e-2 * t81940 - 35.0 / 36.0 * t81943 + 3.0 / 8.0 * t81946 + 0.50869672678616892474e-1 * t81949 - t84932 - 7.0 / 8.0 * t81957 - t81960 / 2.0 - 0.35608770875031824732e0 * t81964 - 0.13565246047631171326e0 * t81972;
    (t84937,)
}
