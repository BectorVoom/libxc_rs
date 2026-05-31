//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2262/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2262<F: Float>(t16949: F, t221: F, t25154: F, t25119: F, t841: F, t81921: F, t81928: F, t81934: F, t81943: F, t81955: F, t87444: F, t87445: F, t87464: F, t87478: F, t87488: F, t98847: F, t98849: F, t98851: F, t98853: F, t98858: F, t98862: F) -> F {
    let t98868 = t25154 * t221 * t16949;
    let t98871 = t25119 * t841 * t16949;
    let t98873 = t98847 / F::cast_from(384.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t98849 + t98851 / F::cast_from(192.0_f64) - t98853 / F::cast_from(768.0_f64) - t81921 + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t81928 - F::cast_from(0.20186378047070195427e-3_f64) * t98858 + F::cast_from(0.12111826828242117256e-2_f64) * t98862 - F::cast_from(0.67826230238155856634e-1_f64) * t81934 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t81943 + t87444 + F::cast_from(0.20186378047070195427e-3_f64) * t87445 - t81955 - t87464 + t98868 / F::cast_from(16.0_f64) + F::cast_from(0.84782787797694820792e-2_f64) * t98871 - t87478 + t87488;
    t98873
}
