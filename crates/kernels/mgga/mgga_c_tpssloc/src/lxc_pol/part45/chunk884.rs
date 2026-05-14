//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 884/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk884<F: Float>(t25: F, t265: F, t394: F, t115099: F, t114991: F, t115040: F, t2250: F, t31478: F, t40: F, t607: F, t8580: F, t1081: F, t113751: F, t113764: F, t113772: F, t114977: F, t115000: F, t115009: F, t115012: F, t1877: F, t23781: F, t23788: F, t23789: F, t23810: F, t24191: F, t24339: F, t26563: F, t26756: F, t31430: F, t31434: F, t31441: F, t31448: F, t31504: F, t4314: F, t7114: F, t83555: F, t84791: F, t8566: F, t8586: F, t89849: F, t89953: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t115100 = piecewise3(t395, 0.0, t115099);
    let t115107 = piecewise3(t115, t114991 + t115040, t115100 * t40 / 2.0 + t31478 * t607 + t8580 * t2250 / 2.0);
    let t115143 = -3.0 / 2.0 * t24191 * t23788 * t115000 + 3.0 * t4314 * t8566 * t23781 - t1877 * t84791 * t8586 / 2.0 - t1877 * t31434 * t23810 - 3.0 * t24191 * t83555 * t31441 - 3.0 * t115009 * t23789 - 3.0 * t26756 * t89953 * t115012 - t1877 * t7114 * t113751 + 2.0 * t26756 * t113764 + 2.0 * t26756 * t89849 * t31448 - 3.0 * t26563 * t23788 * t114977 - t1877 * t24339 * t31504 - 3.0 * t24191 * t113772 + t1877 * t31430 * t1081;
    (t115107, t115143)
}
