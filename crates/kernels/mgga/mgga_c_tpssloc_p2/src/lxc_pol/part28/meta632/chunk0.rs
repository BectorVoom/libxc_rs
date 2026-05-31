//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1989/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1989<F: Float>(t87709: F, t87714: F, t87718: F, t87729: F, t87733: F, t13453: F, t2613: F, t26656: F, t26657: F, t2679: F, t4162: F, t4182: F, t4281: F, t4291: F, t7104: F, t7839: F, t82028: F, t82032: F, t82039: F, t85027: F, t87692: F, t87699: F, t87705: F, t87726: F, t92552: F) -> F {
    let t92810 = F::cast_from(0.76763589786250567036e-1_f64) * t87709;
    let t92811 = F::cast_from(0.9869604401089358619e-1_f64) * t87714;
    let t92817 = F::cast_from(0.10417915756705434098e0_f64) * t87718;
    let t92822 = F::cast_from(0.16449340668482264365e-1_f64) * t87729;
    let t92825 = F::cast_from(0.76763589786250567036e-1_f64) * t87733;
    let t92826 = -F::cast_from(0.16449340668482264365e-1_f64) * t87692 + F::cast_from(0.82246703342411321825e-2_f64) * t82028 + F::cast_from(0.19739208802178717238e0_f64) * t87699 + F::cast_from(0.6579736267392905746e-1_f64) * t87705 - F::cast_from(0.10417915756705434098e0_f64) * t82032 - F::cast_from(0.20835831513410868196e0_f64) * t82039 + t92810 - t92811 + F::cast_from(4.0_f64) * t4281 * t92552 * t4182 + F::cast_from(4.0_f64) * t13453 * t26657 - t85027 - t92817 + t2613 * t7839 + F::cast_from(2.0_f64) * t4162 * t7104 - F::cast_from(0.3289868133696452873e-1_f64) * t87726 + t92822 - t4291 * t26656 * t2679 - t92825;
    t92826
}
