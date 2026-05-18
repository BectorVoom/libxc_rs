//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1305/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1305<F: Float>(t22986: F, t23270: F, t2742: F, t776: F, t857: F, t23273: F, t81591: F, t10112: F, t10115: F, t1912: F, t23281: F, t25168: F, t25169: F, t2720: F, t41554: F, t6627: F, t6663: F, t82087: F, t82092: F, t82099: F, t82108: F, t9590: F) -> F {
    let t82113 = t22986 * t23270 * t857 * t2742 * t776;
    let t82115 = t81591 * t23273;
    let t82117 = -F::new(0.24674011002723396547e-1) * t82087 - F::new(0.9869604401089358619e-1) * t82092 - F::new(18.0) * t25168 * t25169 * t10115 - F::new(3.0) * t41554 * t1912 + F::new(0.78134368175290755733e-1) * t82099 - F::new(3.0) * t9590 * t6663 - F::new(6.0) * t6627 * t10112 + F::new(6.0) * t23281 * t2720 - F::new(0.74022033008170189643e-1) * t82108 + F::new(0.49348022005446793095e-1) * t82113 - F::new(0.23029076935875170111e0) * t82115;
    t82117
}
