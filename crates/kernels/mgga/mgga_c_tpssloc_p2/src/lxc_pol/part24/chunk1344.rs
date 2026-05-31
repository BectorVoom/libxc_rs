//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1344/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1344<F: Float>(t22986: F, t23270: F, t2742: F, t776: F, t857: F, t23273: F, t81591: F, t10112: F, t10115: F, t1912: F, t23281: F, t25168: F, t25169: F, t2720: F, t41554: F, t6627: F, t6663: F, t82087: F, t82092: F, t82099: F, t82108: F, t9590: F) -> F {
    let t82113 = t22986 * t23270 * t857 * t2742 * t776;
    let t82115 = t81591 * t23273;
    let t82117 = -F::cast_from(0.24674011002723396547e-1_f64) * t82087 - F::cast_from(0.9869604401089358619e-1_f64) * t82092 - F::cast_from(18.0_f64) * t25168 * t25169 * t10115 - F::cast_from(3.0_f64) * t41554 * t1912 + F::cast_from(0.78134368175290755733e-1_f64) * t82099 - F::cast_from(3.0_f64) * t9590 * t6663 - F::cast_from(6.0_f64) * t6627 * t10112 + F::cast_from(6.0_f64) * t23281 * t2720 - F::cast_from(0.74022033008170189643e-1_f64) * t82108 + F::cast_from(0.49348022005446793095e-1_f64) * t82113 - F::cast_from(0.23029076935875170111e0_f64) * t82115;
    t82117
}
