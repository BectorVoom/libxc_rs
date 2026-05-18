//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1067/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1067<F: Float>(t2228: F, t2350: F, t903: F, t15467: F, t4601: F, t1550: F, t699: F, t8704: F, t75859: F, t75864: F, t75866: F, t1356: F, t14434: F, t235: F, t5144: F, t515: F, t5267: F, t5888: F, t70048: F, t70050: F, t71158: F, t71661: F, t739: F, t75869: F, t75874: F, t75876: F, t75881: F, t78184: F, t884: F) -> F {
    let t78321 = t903 * t2228 * t2350;
    let t78322 = F::new(0.44903406381989282115e-1) * t78321;
    let t78323 = t4601 * t15467;
    let t78324 = F::new(0.44903406381989282115e-1) * t78323;
    let t78326 = t1550 * t699 * t8704;
    let t78327 = F::new(0.2993560425465952141e-1) * t78326;
    let t78339 = F::new(0.44903406381989282115e-1) * t75859;
    let t78340 = F::new(0.38430329123504567781e-4) * t75864;
    let t78341 = F::new(0.38430329123504567781e-4) * t75866;
    let t78348 = -t78322 - t78324 + t78327 + F::new(0.11974241701863808564e0) * t739 * t14434 * t5144 - F::new(0.11974241701863808564e0) * t884 * t14434 * t5267 - F::new(0.11974241701863808564e0) * t1356 * t71158 * t5888 - F::new(0.57000320883372412496e-7) * t70048 - F::new(0.57000320883372412496e-7) * t70050 + t71661 - t78339 + t78340 + t78341 + F::new(0.76860658247009135557e-5) * t75869 - F::new(0.19957069503106347607e-1) * t235 * t515 * t78184 + t75874 + F::new(0.6505345598561924296e-5) * t75876 + F::new(0.6505345598561924296e-5) * t75881;
    t78348
}
