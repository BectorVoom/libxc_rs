//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 665/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk665<F: Float>(t2227: F, t558: F, t1587: F, t698: F, t2447: F, t321: F, t333: F, t623: F, t8619: F, t511: F, t6477: F, t2144: F, t892: F) -> (F, F, F, F, F, F, F) {
    let t44232 = t2227 * t558;
    let t44239 = t698 * t1587;
    let t44244 = t2447 * t321;
    let t44293 = t2447 * t333;
    let t44788 = t623 * t8619;
    let t45468 = t6477 * t511;
    let t52781 = t892 * t2144;
    (t44232, t44239, t44244, t44293, t44788, t45468, t52781)
}
