//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1106/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1106<F: Float>(t28354: F, t28430: F, t858: F, t218: F, t28406: F, t25224: F, t7488: F, t1880: F, t1492: F, t7510: F, t17090: F, t1912: F, t23231: F, t23252: F, t23262: F, t25206: F, t25209: F, t259: F, t26712: F, t26726: F, t28307: F, t28311: F, t28317: F, t4268: F, t5637: F, t5658: F, t6627: F, t7538: F, t855: F) -> (F, F, F, F, F, F) {
    let t28431 = t28354 + t28430;
    let t28432 = t858 * t28431;
    let t28437 = t218 * t28406;
    let t28439 = t25224 * t7488;
    let t28440 = t1880 * t28439;
    let t28442 = t1492 * t7510;
    let t28446 = -t23231 - t6627 * t5658 + 4.0 * t855 * t28307 - 6.0 * t855 * t28311 - 2.0 * t4268 * t7538 + 2.0 * t855 * t28317 + 0.82246703342411321824e-2 * t25206 - t855 * t28432 + 0.76763589786250567036e-1 * t25209 + t26712 + 2.0 * t6627 * t5637 + t28437 * t259 + t23252 + t23262 - 0.16449340668482264365e-1 * t28440 + 2.0 * t28442 * t259 + t26726 - t17090 * t1912;
    (t28431, t28432, t28437, t28439, t28442, t28446)
}
