//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1172/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1172<F: Float>(t18178: F, t5642: F, t2703: F, t342: F, t345: F, t5640: F, t18173: F, t1731: F, t18130: F, t347: F, t1730: F, t1733: F, t18131: F, t18133: F, t18140: F, t18142: F, t18145: F, t18152: F, t18156: F, t18158: F, t18162: F, t18166: F, t18171: F, t18175: F, t2778: F, t2805: F, t373: F, t5626: F, t5629: F, t5631: F, t5634: F, t5639: F, t5643: F, t5646: F, t991: F) -> (F, F, F, F, F, F, F) {
    let t18179 = t18178 * t5642;
    let t18183 = t2703 * t342 * t345;
    let t18184 = t5640 * t18183;
    let t18186 = t18173 * t345;
    let t18187 = t5640 * t18186;
    let t18190 = t1731 * t347 * t18130;
    let t18192 = -t1730 * t18190 - t1733 * t18140 + t18131 * t373 - 2.0 * t18133 * t991 + 4.0 * t18142 * t5634 - 2.0 * t18145 * t5643 - 6.0 * t18152 * t5631 + 4.0 * t18156 * t18158 + 4.0 * t18162 * t5631 + 2.0 * t18166 * t5631 - 2.0 * t18171 * t18175 + t18171 * t18187 - 2.0 * t18179 * t5639 - t18184 * t5639 + 2.0 * t2778 * t5626 - t2805 * t5626 - 2.0 * t5629 * t5646;
    (t18179, t18183, t18184, t18186, t18187, t18190, t18192)
}
