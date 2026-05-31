//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1033/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1033<F: Float>(t117407: F, t117410: F, t117412: F, t117416: F, t117418: F, t117420: F, t117422: F, t117430: F, t117662: F, t117671: F, t117690: F, t1396: F, t1398: F, t1404: F, t2099: F, t2105: F, t2170: F, t2174: F, t24448: F, t24486: F, t24955: F, t24977: F, t3: F, t32393: F, t32415: F, t3932: F, t3946: F, t580: F, t7223: F, t7240: F, t7416: F, t7426: F, t8844: F, t8852: F) -> F {
    let tv4rho2sigma24 = F::cast_from(2.0_f64) * t117407 + t3932 * t8852 + F::cast_from(2.0_f64) * t117410 + F::cast_from(2.0_f64) * t117412 + F::cast_from(2.0_f64) * t7416 * t7240 + F::cast_from(2.0_f64) * t117416 + F::cast_from(2.0_f64) * t117418 + F::cast_from(2.0_f64) * t117420 + F::cast_from(2.0_f64) * t117422 + t2170 * t24486 + F::cast_from(2.0_f64) * t32393 * t1404 + F::cast_from(2.0_f64) * t7223 * t7426 + t8844 * t3946 + F::cast_from(2.0_f64) * t117430 + t24955 * t2105 + t24448 * t2174 + t2099 * t24977 + F::cast_from(2.0_f64) * t1396 * t32415 + t3 * t117662 * t580 + t1398 * (t117671 + t117690);
    tv4rho2sigma24
}
