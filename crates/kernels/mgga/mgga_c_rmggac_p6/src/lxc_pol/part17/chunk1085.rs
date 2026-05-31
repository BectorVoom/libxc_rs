//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1085/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1085<F: Float>(t10007: F, t1356: F, t30490: F, t36280: F, t39320: F, t41978: F, t41980: F, t45556: F, t4601: F, t46072: F, t47757: F, t47759: F, t47761: F, t47765: F, t47767: F, t47772: F, t47774: F, t47785: F, t47787: F, t4985: F, t530: F, t7703: F, t884: F, t8866: F) -> F {
    let t47791 = -F::cast_from(0.4726e1_f64) * t530 * t39320 + F::cast_from(0.11974241701863808564e0_f64) * t4985 * t8866 - F::cast_from(0.85129199786595678796e-5_f64) * t47757 + t41978 - t41980 + F::cast_from(0.18183107769496894486e-1_f64) * t47759 + t47761 + F::cast_from(0.15961724959986689774e-4_f64) * t47765 + F::cast_from(0.1064114997332445985e-4_f64) * t47767 + F::cast_from(0.1064114997332445985e-4_f64) * t47772 + F::cast_from(0.47896966807455234256e0_f64) * t47774 + F::cast_from(0.79828278012425390428e-1_f64) * t1356 * t46072 + F::cast_from(0.35922725105591425692e0_f64) * t884 * t7703 * t30490 + F::cast_from(0.47896966807455234256e0_f64) * t1356 * t36280 * t45556 + F::cast_from(0.40911992481368012592e-1_f64) * t47785 - F::cast_from(0.81823984962736025184e-1_f64) * t47787 + F::cast_from(0.35922725105591425692e0_f64) * t4601 * t10007;
    t47791
}
