//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 942/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk942<F: Float>(t1931: F, t1986: F, t7720: F, t1356: F, t1632: F, t2024: F, t2402: F, t289: F, t39591: F, t45614: F, t45617: F, t45622: F, t45626: F, t45630: F, t45633: F, t45636: F, t45641: F, t45646: F, t45648: F, t45651: F, t45656: F, t45660: F, t45664: F, t884: F, t903: F) -> F {
    let t45666 = t1986 * t1931;
    let t45667 = t7720 * t45666;
    let t45669 = F::cast_from(0.25538759935978703638e-4_f64) * t45614 + F::cast_from(0.25538759935978703638e-4_f64) * t45617 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2402 * t1632 - F::cast_from(0.11974241701863808564e0_f64) * t884 * t2024 * t45622 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t45626 - F::cast_from(0.40650199722100037752e-3_f64) * t45630 - F::cast_from(0.81300399444200075504e-3_f64) * t45633 - F::cast_from(0.40650199722100037752e-3_f64) * t45636 + F::cast_from(0.1064114997332445985e-4_f64) * t45641 - F::cast_from(0.1064114997332445985e-4_f64) * t45646 - F::cast_from(0.25538759935978703638e-4_f64) * t45648 - F::cast_from(0.74488049813271218946e-4_f64) * t39591 - F::new(0.2363e1) * t289 * t45651 - F::cast_from(0.51077519871957407276e-4_f64) * t45656 + F::cast_from(0.76616279807936110914e-4_f64) * t45660 + F::cast_from(0.51077519871957407276e-4_f64) * t45664 - F::cast_from(0.25538759935978703638e-4_f64) * t45667;
    t45669
}
