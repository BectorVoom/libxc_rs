//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 816/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk816<F: Float>(t26291: F, t77327: F, t14444: F, t1635: F, t29838: F, t5898: F, t34813: F, t5144: F, t40724: F, t5267: F, t235: F, t26087: F, t5888: F, t15516: F, t4965: F, t74984: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77329 = 0.35922725105591425692e0 * t26291 * t77327;
    let t77330 = t14444 * t1635;
    let t77332 = 0.47896966807455234256e0 * t29838 * t77330;
    let t77335 = t14444 * t5898;
    let t77337 = 0.35922725105591425692e0 * t34813 * t77335;
    let t77338 = t14444 * t5144;
    let t77340 = 0.35922725105591425692e0 * t40724 * t77338;
    let t77341 = t14444 * t5267;
    let t77343 = 0.35922725105591425692e0 * t34813 * t77341;
    let t77344 = t235 * t26087;
    let t77345 = t14444 * t5888;
    let t77347 = 0.47896966807455234256e0 * t77344 * t77345;
    let t77349 = 0.39914139006212695214e-1 * t4965 * t15516;
    let t77352 = 0.40911992481368012592e-1 * t74984;
    (t77329, t77330, t77332, t77335, t77337, t77338, t77340, t77341, t77343, t77345, t77347, t77349, t77352)
}
