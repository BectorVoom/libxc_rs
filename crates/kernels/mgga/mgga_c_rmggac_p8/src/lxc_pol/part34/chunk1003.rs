//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1003/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1003<F: Float>(t77528: F, t1971: F, t3351: F, t44157: F, t875: F, t44183: F, t69648: F, t69663: F, t69664: F, t69666: F, t71419: F, t71429: F, t75248: F, t77514: F, t77515: F, t77517: F, t77519: F, t77520: F, t77521: F, t77525: F, t884: F) -> F {
    let t77529 = F::cast_from(0.85129199786595678796e-5_f64) * t77528;
    let t77532 = t3351 * t1971 * t875 * t44157;
    let t77533 = F::cast_from(0.85129199786595678796e-5_f64) * t77532;
    let t77536 = t3351 * t1971 * t875 * t44183;
    let t77537 = F::cast_from(0.85129199786595678796e-5_f64) * t77536;
    let t77538 = -t77514 - t77515 - t77517 - t77519 - t77520 - t77521 + t71419 - F::cast_from(0.40878380883436523436e-5_f64) * t69648 - t69663 + F::cast_from(0.24527028530061914063e-5_f64) * t69664 - F::cast_from(0.24527028530061914063e-5_f64) * t69666 + F::cast_from(0.59871208509319042821e-1_f64) * t884 * t77525 - t71429 + t75248 + t77529 + t77533 + t77537;
    t77538
}
