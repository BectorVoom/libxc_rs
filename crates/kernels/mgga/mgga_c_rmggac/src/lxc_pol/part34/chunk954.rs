//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 954/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk954<F: Float>(t76924: F, t14672: F, t17859: F, t74219: F, t14551: F, t8368: F, t74232: F, t74199: F, t74207: F, t74209: F, t74213: F, t74217: F, t74225: F, t74228: F, t74235: F, t76904: F, t76913: F, t76918: F, t76923: F) -> F {
    let t76925 = F::new(0.42564599893297839398e-5) * t76924;
    let t76926 = t17859 * t14672;
    let t76927 = F::new(0.12769379967989351819e-4) * t76926;
    let t76928 = F::new(0.1921128438866447784e-2) * t74219;
    let t76930 = t8368 * t14551;
    let t76931 = F::new(0.90915538847484472429e-2) * t76930;
    let t76932 = F::new(0.68186654135613354325e-2) * t74232;
    let t76934 = -F::new(0.57000320883372412496e-7) * t74199 + t76904 + F::new(0.58171619854173713846e-5) * t74207 - F::new(0.58171619854173713846e-5) * t74209 + F::new(0.58171619854173713846e-5) * t74213 - F::new(0.17451485956252114154e-4) * t74217 - t76913 + t76918 + t76923 + t76925 - t76927 + t76928 - t74225 + F::new(0.70077224371605468752e-6) * t74228 + t76931 - t76932 + F::new(0.35038612185802734376e-6) * t74235;
    t76934
}
