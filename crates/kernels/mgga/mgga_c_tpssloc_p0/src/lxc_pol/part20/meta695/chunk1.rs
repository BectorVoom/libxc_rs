//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2648/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2648<F: Float>(t40018: F, t5223: F, t16379: F, t40021: F, t12156: F, t12240: F, t12353: F, t12407: F, t1307: F, t1369: F, t16225: F, t16305: F, t16306: F, t16321: F, t16355: F, t1810: F, t210: F, t3733: F, t3803: F, t3876: F, t39936: F, t40025: F, t5240: F, t5246: F, t53907: F, t53910: F, t53918: F, t53920: F, t53921: F) -> F {
    let t53927 = t40018 * t5223;
    let t53928 = F::new(35.0) / F::new(24.0) * t53927;
    let t53929 = t40021 * t16379;
    let t53943 = F::new(7.0) / F::new(192.0) * t53907 - t53910 * t1369 / F::new(256.0) - t16321 * t3876 / F::new(256.0) - F::new(5.0) / F::new(128.0) * t5240 * t12353 - t53918 - t53920 - F::new(7.0) / F::new(8.0) * t53921 + F::new(3.0) / F::new(16.0) * t3733 * t210 * t16355 * t1307 + t53928 + F::new(7.0) / F::new(4.0) * t53929 + t39936 + F::new(5.0) / F::new(4.0) * t40025 * t210 * t1810 * t12156 - t5246 * t16305 * t16225 * t12240 / F::new(128.0) + t3803 * t16305 * t16306 * t12407 / F::new(256.0);
    t53943
}
