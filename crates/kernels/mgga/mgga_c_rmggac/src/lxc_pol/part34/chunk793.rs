//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 793/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk793<F: Float>(t73814: F, t73819: F, t73822: F, t73837: F, t73845: F, t1971: F, t3351: F, t4617: F, t9552: F, t15450: F, t7244: F, t495: F, t7230: F, t875: F, t9551: F, t73807: F, t73812: F, t73817: F, t73827: F, t73833: F, t73840: F, t73843: F, t73847: F, t73849: F) -> (F,) {
    let t76688 = 0.16351352353374609375e-5 * t73814;
    let t76689 = 0.39726959900411316773e-4 * t73819;
    let t76690 = 0.2553875993597870364e-4 * t73822;
    let t76693 = 0.2553875993597870364e-4 * t73837;
    let t76696 = 0.23268647941669485538e-4 * t73845;
    let t76700 = t3351 * t1971 * t4617 * t9552;
    let t76701 = 0.25538759935978703639e-4 * t76700;
    let t76702 = t7244 * t15450;
    let t76703 = 0.99317399751028291929e-5 * t76702;
    let t76707 = t7230 * t1971 * t875 * t9551 * t495;
    let t76708 = 0.1064114997332445985e-4 * t76707;
    let t76709 = -0.58171619854173713846e-5 * t73807 - 0.58171619854173713846e-5 * t73812 + t76688 + t73817 - t76689 + t76690 - 0.4379826523225341797e-6 * t73827 - 0.1532939283128869629e-5 * t73833 - t76693 - 0.8759653046450683594e-6 * t73840 + 0.13139479569676025391e-5 * t73843 - t76696 - 0.58171619854173713846e-5 * t73847 - t73849 - t76701 - t76703 - t76708;
    (t76709,)
}
