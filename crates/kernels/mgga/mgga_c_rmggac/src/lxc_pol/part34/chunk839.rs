//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 839/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk839<F: Float>(t75088: F, t75096: F, t14618: F, t8368: F, t14421: F, t2868: F, t75119: F, t75124: F, t2010: F, t2265: F, t8342: F, t2415: F, t8048: F, t8188: F, t14434: F, t5898: F, t75092: F, t75100: F, t75103: F, t75106: F, t75108: F, t75110: F, t75115: F, t884: F) -> (F,) {
    let t77450 = 0.36366215538993788973e-1 * t75088;
    let t77452 = 0.11634323970834742769e-4 * t75096;
    let t77457 = t8368 * t14618;
    let t77458 = 0.34093327067806677161e-2 * t77457;
    let t77463 = 0.11974241701863808564e0 * t2868 * t14421;
    let t77464 = 0.1702583995731913576e-4 * t75119;
    let t77465 = 0.85129199786595678799e-5 * t75124;
    let t77467 = t2010 * t8342 * t2265;
    let t77468 = 0.36021158228745895953e-3 * t77467;
    let t77470 = t2010 * t2415 * t8048;
    let t77471 = 0.36021158228745895953e-3 * t77470;
    let t77473 = t2010 * t2415 * t8188;
    let t77474 = 0.36021158228745895953e-3 * t77473;
    let t77475 = -t77450 - 0.8759653046450683594e-6 * t75092 + t77452 - 0.58171619854173713846e-5 * t75100 - 0.72714524817717142308e-5 * t75103 - 0.10511583655740820313e-5 * t75106 - 0.58171619854173713846e-5 * t75108 - t77458 + t75110 + t75115 - 0.11974241701863808564e0 * t884 * t14434 * t5898 + t77463 - t77464 + t77465 - t77468 - t77471 - t77474;
    (t77475,)
}
