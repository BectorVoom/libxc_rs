//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1450/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1450<F: Float>(t104015: F, t104282: F, t104294: F, t104296: F, t104337: F, t1737: F, t2134: F, t22012: F, t22032: F, t22185: F, t24815: F, t24821: F, t27614: F, t27617: F, t27636: F, t27637: F, t27642: F, t29644: F, t29648: F, t460: F, t6203: F, t6218: F, t6221: F, t7310: F, t7320: F, t7345: F, t8040: F, t95387: F, t95512: F, t95520: F) -> F {
    let t109627 = t104294 / F::new(384.0) + t104296 / F::new(384.0) + t95512 / F::new(432.0) - F::new(0.30279567070605293142e-3) * t27636 * t27642 * t24821 * t6218 + F::new(0.30279567070605293142e-3) * t95387 * t29648 - F::new(0.30279567070605293142e-3) * t104282 * t8040 + F::new(0.60559134141210586284e-3) * t27636 * t27637 * t24815 * t6218 - F::new(0.60559134141210586284e-3) * t95387 * t29644 - F::new(0.30279567070605293142e-3) * t104337 + t95520 / F::new(432.0) - F::new(0.10093189023535097714e-3) * t2134 * t22032 * t460 * t7320 - F::new(7.0) / F::new(648.0) * t7310 * t22012 + F::new(5.0) / F::new(2304.0) * t27617 * t6203 + F::new(5.0) / F::new(1152.0) * t7345 * t22185 + t104015 * t1737 / F::new(512.0) + t27614 * t6221 / F::new(512.0);
    t109627
}
