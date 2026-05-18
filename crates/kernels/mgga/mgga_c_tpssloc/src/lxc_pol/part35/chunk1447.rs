//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1447/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1447<F: Float>(t22298: F, t3032: F, t104012: F, t104107: F, t104111: F, t104120: F, t104124: F, t104190: F, t104300: F, t11721: F, t1737: F, t1748: F, t22271: F, t24729: F, t27617: F, t29644: F, t29648: F, t3508: F, t475: F, t6211: F, t8040: F, t86155: F, t86157: F, t86191: F, t86208: F, t86214: F, t95295: F, t95365: F) -> F {
    let t109505 = t22298 * t3032;
    let t109528 = t104300 * t1748 / F::new(72.0) + F::new(19.0) / F::new(288.0) * t104012 * t1737 - F::new(19.0) / F::new(432.0) * t104107 * t1748 + F::new(0.21801288290835811062e-1) * t104190 * t8040 - F::new(0.60559134141210586284e-3) * t95295 * t29644 + F::new(0.60559134141210586284e-3) * t86155 * t86208 * t109505 * t11721 - F::new(0.60559134141210586284e-3) * t86155 * t86214 * t109505 * t3508 + F::new(0.30279567070605293142e-3) * t95295 * t29648 + F::new(0.10093189023535097714e-3) * t86155 * t86157 * t109505 * t475 - t95365 / F::new(2304.0) + t86191 + F::new(0.48447307312968469026e-2) * t104111 + F::new(0.60559134141210586284e-3) * t104120 - F::new(0.30279567070605293142e-3) * t104124 + t24729 * t22271 / F::new(256.0) - t27617 * t6211 / F::new(384.0);
    t109528
}
