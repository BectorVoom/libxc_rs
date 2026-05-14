//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 867/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk867<F: Float>(t4669: F, t558: F, t71903: F, t71949: F, t71940: F, t326: F, t650: F, t9565: F, t333: F, t352: F, t5155: F, t5266: F, t69181: F, t69183: F, t76212: F, t76216: F, t76218: F, t76222: F, t76224: F, t77890: F) -> (F,) {
    let t77916 = t4669 * t71903 * t558;
    let t77917 = 0.44903406381989282115e-1 * t77916;
    let t77919 = t4669 * t71949 * t558;
    let t77920 = 0.11974241701863808564e0 * t77919;
    let t77921 = 0.39914139006212695213e-1 * t71940;
    let t77929 = t326 * t9565 * t650;
    let t77930 = 0.34093327067806677161e-2 * t77929;
    let t77931 = t77917 - t77920 + t77921 + 0.23948483403727617128e0 * t5155 * t77890 * t333 + 0.11974241701863808564e0 * t5266 * t77890 * t352 + t76212 - t76216 - t76218 - t76222 - t76224 - t69181 - t69183 + t77930;
    (t77931,)
}
